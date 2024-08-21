use std::collections::HashMap;

use rocket::serde::{Serialize, Deserialize, json::Json};
use rocket::serde::json::{Value, json};
use rocket::response::{status, status::Custom};
use rocket::http::Status;

use diesel::sql_query;
use diesel::sql_types::*;

use rocket_db_pools::{Database, Connection};
use rocket_db_pools::diesel::{MysqlPool, prelude::*};

use crate::global::{generate_random_id, get_epoch, is_null_or_whitespace, request_authentication};
use crate::internal::folder::{folder_get, folder_list};
use crate::internal::item::index::item_get;
use crate::internal::keyword::keyword::keywords_from_text;
use crate::responses::*;
use crate::structs::*;
use crate::tables::*;
use crate::SQL_TABLES;

#[get("/list?<item>")]
pub async fn item_content_list(mut db: Connection<Db>, item: Option<String>, params: &Query_string) -> Custom<Value> {
    let sql: Config_sql = (&*SQL_TABLES).clone();

    if (is_null_or_whitespace(item.clone()) == true) {
        return status::Custom(Status::NotFound, error_message("param.item is null or whitespace."));
    }
    let item_id: String = item.unwrap();

    let request_authentication_output: Request_authentication_output = match request_authentication(db, None, params, "/item/content/list", false).await {
        Ok(data) => data,
        Err(e) => return status::Custom(Status::Unauthorized, not_authorized())
    };
    db = request_authentication_output.returned_connection;

    // Get the item. This will check this person has permission to access the item (because we passed a user_id).
    let (item_data, error_to_respond_with, folder_db) = crate::internal::item::index::item_get(db, item_id.clone(), Some(request_authentication_output.user_id.clone())).await.expect("Error looking up folder.");
    db = folder_db;

    if (error_to_respond_with.is_none() == false) {
        return status::Custom(Status::BadRequest, error_to_respond_with.unwrap());
    }
    if (item_data.is_none() == true) {
        return status::Custom(Status::NotFound, error_message("param.item was not found, or you are not authorized to access it."));
    }
    let item_data_public: Mindmap_item_public = item_data.unwrap().into();

    // TODO: Return item_content_result as Option<Vec<Mindmap_item_content>> - for example, I forgot error_to_respond_with here at one point, and it would have just returned and empty result with no reliability. This should be added to everything else as well.
    let (item_content_result, error_to_respond_with, content_list_db) = crate::internal::item::content::content_list(db, item_id.clone(), request_authentication_output.user_id.clone()).await.expect("Failed to get item content list.");
    db = content_list_db;

    if (error_to_respond_with.is_none() == false) {
        return status::Custom(Status::BadRequest, error_to_respond_with.unwrap());
    }

    // Return public data, not internal system or admin data.
    let mut item_content_result_public: Vec<Mindmap_item_content_public> = item_content_result
    .into_iter()
    .map(Mindmap_item_content_public::from)
    .collect();

    // We need to get keywords for this document, instead of trying to write some fancy but messy SQL query to do this (including dealing with chunks in the SQL query), we can instead take any returned text, put it into an array, join it with spaces, and get the same result.
    let mut texts: Vec<String> = Vec::new();
    for item in item_content_result_public.clone() {
        texts.push(item.content.unwrap());
    }

    // Get keywords for this item, using the joined text. This will use SQL's LIKE to find relevant keywords used here.
    let (keywords_for_item, error_to_respond_with, keywords_from_text_db) = keywords_from_text(db, texts.join(" "), request_authentication_output.user_id.clone()).await.expect("Failed to get keywords from text");
    db = keywords_from_text_db;

    if (error_to_respond_with.is_none() == false) {
        return status::Custom(Status::BadRequest, error_to_respond_with.unwrap());
    }

    // Return public data, not internal system or admin data.
    let mut keywords_for_item_public: Vec<Rendered_keyword_public> = keywords_for_item
    .into_iter()
    .map(Rendered_keyword_public::from)
    .collect();

    status::Custom(Status::Ok, json!({
        "ok": true,
        "item": item_data_public,
        "data": item_content_result_public,
        "keywords": keywords_for_item_public
    }))
}

#[post("/update", format = "application/json", data = "<body>")]
pub async fn item_content_update(mut db: Connection<Db>, params: &Query_string, mut body: Json<Item_content_update_body>) -> Custom<Value> {
    let sql: Config_sql = (&*SQL_TABLES).clone();

    let item_content_table = sql.item_content.unwrap();
    
    let request_authentication_output: Request_authentication_output = match request_authentication(db, None, params, "/item/content/update", false).await {
        Ok(data) => data,
        Err(e) => return status::Custom(Status::Unauthorized, not_authorized())
    };
    db = request_authentication_output.returned_connection;

    // Validate inputs before writing all the incoming data.

    // When an item is run through item_get, we cache it here. So if there are 50 items, we don't call item_get 50 tiems.
    // It is very important item_id and user_id are checked before using a cache as auth for an item, and if get_item changes to any other potential conditions, it's important those are checked in the future as well.
    let mut authed_items: HashMap<Item_content_Authed_item_cache_check, Mindmap_item> = HashMap::new();

    // These are actions that have already had their data validated.
    let mut verified_queued_actions: Vec<Item_content_action> = Vec::new();

    if (body.actions.is_none() == true) {
        return status::Custom(Status::BadRequest, error_message("body.actions must be specified."));
    }
    let actions = body.actions.clone().expect("Missing body.actions");
    if (actions.len() > 10000) {
        return status::Custom(Status::BadRequest, error_message("body.actions cannot have more than 100 actions."));
    }

    for action_data in actions.iter() {
        let mut action = action_data.clone();
        // TODO: There should be an action logic pipeline.
        // Normallly it would matter what the value of unwrap_or was here, since we're trying to check the original value, in this case checking if it's None, but it doesn't matter here because there is a check for 'create' or 'update'.
        let action_type = action.action.clone().unwrap_or(String::new());
        if (action_type != "create" && action_type != "update") {
            return status::Custom(Status::BadRequest, error_message("body.actions.action must be create/update."));
        }
        if (action_type == "update" && is_null_or_whitespace(action.row_id.clone()) == true) {
            return status::Custom(Status::BadRequest, error_message("body.actions.id must be specified when body.actions.action='update'"));
        }
        // if (action_type == "create" && is_null_or_whitespace(action.row_id.clone()) == false) {
        //     return status::Custom(Status::BadRequest, error_message("body.actions.id cannot be specified when body.actions.action='create'"));
        // }

        if (is_null_or_whitespace(action.item.clone()) == true) {
            return status::Custom(Status::BadRequest, error_message("body.actions.item is null or whitespace."));
        }
        // We don't care if parent is null.
        // if (is_null_or_whitespace(action.parent.clone()) == true) {
        //     return status::Custom(Status::BadRequest, error_message("body.actions.parent is null or whitespace."));
        // }
        if (action.rank.is_none() == true) {
            return status::Custom(Status::BadRequest, error_message("body.actions.rank is null or whitespace."));
        }
        if (is_null_or_whitespace(action.content.clone()) == true) {
            return status::Custom(Status::BadRequest, error_message("body.actions.content is null or whitespace."));
        }

        let item_id = action.item.clone().expect("missing action.item");
        let parent = action.parent.clone();
        let rank = action.rank.clone().expect("missing action.rank");
        let content = action.content.clone().expect("missing action.content");
        // let row_id = action.row_id.clone().expect("missing action.row_id");

        if (content.len() > 100000) {
            return status::Custom(Status::BadRequest, error_message("action.content is over 100,000 characters. This server has a limit of 100,000 characters."));
        }

        // TODO: To prevent collisions, custom IDs should have a nonce. Otherwise an asset with the same ID but different data could get mixed up mid-request, for example, if a deletion occured, it may not be for the occupying item.
        if (action_type == "create") {
            if (action.row_id.is_none() == false) {
                if (action.row_id.clone().unwrap().starts_with(&format!("{}_", item_id.clone())) == false) {
                    return status::Custom(Status::BadRequest, error_message(&format!("action.row_id - custom row_id(s) must be formatted as [item]_[custom_id]. For example, here, your row_id here should have been {}", format!("{}_{}", item_id.clone(), action.row_id.clone().unwrap()))));
                }
                // Prepend the custom row_id for safety.
                action.row_id = Some(action.row_id.clone().unwrap());
                // TODO: Check id isn't already being used.
            } else {
                action.row_id = Some(generate_random_id());
            }
        }

        // TODO: IF parent exists, check the parent actually exists!
        // TODO: If row_id exists, maybe we should check if the row actually exists? depends if you're doing anything else as a side effect of that row existing or not.

        // ---- Verify item and verify this user has access. ----
        let cache_check = Item_content_Authed_item_cache_check {
            item_id: item_id.clone(),
            user_id: request_authentication_output.user_id.clone()
        };

        let item_data: Option<Mindmap_item>;
        if let Some(item_status) = authed_items.get(&cache_check) {
            println!("Item ID already authed for this batch: {}", item_id.clone());
            item_data = Some(item_status.clone());
        } else {
            let (item_status, error_to_respond_with, folder_db) = item_get(db, item_id.clone(), Some(request_authentication_output.user_id.clone())).await.expect("Error looking up item.");
            db = folder_db;

            if (error_to_respond_with.is_none() == false) {
                return status::Custom(Status::BadRequest, error_to_respond_with.unwrap());
            }
            if (item_status.is_none() == true) {
                return status::Custom(Status::NotFound, error_message("action.folder was not found, or you are not authorized to access it."));
            }
            
            authed_items.insert(cache_check, item_status.clone().unwrap());

            item_data = Some(item_status.expect("Something went wrong, got an empty item_status, when it should be fully cached."));
        }

        // Very important for ensuring authentication was successful, and the item actually exists.
        item_data.expect("Item data should have been returned.");
        // ----

        verified_queued_actions.push(action.clone());
    }

    for user_input in verified_queued_actions.iter() {
        // user_input.row_id has overwritten by the validation function before. DATA IN user_input IS STILL UNTRUSTED. Data there doesn't mean it has 100% integrity, it just means it's passed the initial validation checks. It's not system-data.

        // This value is overwritten by the value before, which is why it's ok. Though it will only get overwritten if the action is "create".
        let mut row_id = user_input.row_id.clone().expect("Missing user_input.row_id");

        let action_type = user_input.action.clone().expect("missing user_input.action");
        let item_id = user_input.item.clone().expect("missing user_input.item");
        let parent = user_input.parent.clone(); // we don't care if parent exists or not, we just care it's valid if provided. TODO: maybe validate parent exists?
        let rank = user_input.rank.clone().expect("missing user_input.rank");
        let content = user_input.content.clone().expect("missing user_input.content");

        if (action_type == "update") {
            // We know 'user_input.id' exists, because we checked when validating the 'user_input.action'.
            row_id = user_input.row_id.clone().expect("missing user_input.row_id");

            let (item, error_to_respond_with, item_db) = item_get(db, item_id.clone(), Some(request_authentication_output.user_id.clone())).await.expect("Failed to get folder.");  
            db = item_db;
            
            if (item.is_none() == true) {
                return status::Custom(Status::BadRequest, error_message(&format!("Item does not exist: '{}'", item_id.clone())));
            }

            // format!() is not for values. It uses heavily vetted and sanitized values that are directly from the admin's configuration on local environment variables.
            sql_query(&format!("UPDATE {} SET parent=?, rank=?, content=? WHERE row_id=? AND item=?", item_content_table))
            // .bind::<Text, _>(item_id.clone())
            .bind::<Nullable<Text>, _>(parent.clone())
            .bind::<BigInt, _>(rank.clone())
            .bind::<Text, _>(content.clone())
            .bind::<Text, _>(row_id.clone())
            .bind::<Text, _>(item_id.clone())
            .execute(&mut db)
            .await
            .expect("Something went wrong querying the DB.");
        } else if (action_type == "create") {
            // format!() is not for values. It uses heavily vetted and sanitized values that are directly from the admin's configuration on local environment variables.
            sql_query(&format!("INSERT INTO {} (row_id, parent, item, rank, content, created) VALUES (?, ?, ?, ?, ?, ?)", item_content_table))
            .bind::<Text, _>(row_id.clone())
            .bind::<Nullable<Text>, _>(parent.clone())
            .bind::<Text, _>(item_id.clone())
            .bind::<BigInt, _>(rank.clone())
            .bind::<Text, _>(content.clone())
            .bind::<BigInt, _>(get_epoch())
            .execute(&mut db)
            .await
            .expect("Something went wrong querying the DB.");
        }
    }

    return status::Custom(Status::Ok, json!({
        "ok": true
    }));
}