use rocket::serde::{Serialize, Deserialize, json::Json};
use rocket::serde::json::{Value, json};
use rocket::response::{status, status::Custom};
use rocket::http::Status;

use diesel::sql_query;
use diesel::prelude::*;
use diesel::sql_types::*;

use crate::global::{generate_random_id, get_epoch, is_null_or_whitespace, request_authentication};
use crate::internal::folder::folder_get;
use crate::internal::keyword::keyword::keyword_get;
use crate::responses::*;
use crate::structs::*;
use crate::tables::*;
use crate::SQL_TABLES;

use url::Url;

#[get("/list?<ids>")]
pub async fn keyword_list(params: &Query_string, ids: Option<String>) -> Custom<Value> {
    let mut db = crate::DB_POOL.get().expect("Failed to get a connection from the pool.");
    let request_authentication_output: Request_authentication_output = match request_authentication(None, params, "/keyword/list", false).await {
        Ok(data) => data,
        Err(e) => return status::Custom(Status::Unauthorized, not_authorized())
    };

    let mut ids_split: Vec<String> = Vec::new();
    if (ids.is_none() == false) {
        ids_split = ids.unwrap().split(',')
        .map(|s| s.to_string())
        .collect();
    }

    let (rendered_keywords, error_to_respond_with) = crate::internal::keyword::keyword::keyword_list(request_authentication_output.user_id.clone(), Some(ids_split), None).await.expect("Failed to get keyword_list");
    
    if (error_to_respond_with.is_none() == false) {
        return status::Custom(Status::BadRequest, error_to_respond_with.unwrap());
    }

    let rendered_keywords_public: Vec<Rendered_keyword_public> = rendered_keywords
    .into_iter()
    .map(Rendered_keyword_public::from)
    .collect();

    status::Custom(Status::Ok, json!({
        "ok": true,
        "data": rendered_keywords_public
    }))
}

#[post("/update", format = "application/json", data = "<body>")]
pub async fn keyword_update(params: &Query_string, mut body: Json<Keyword_update_body>) -> Custom<Value> {
    let mut db = crate::DB_POOL.get().expect("Failed to get a connection from the pool.");
    let sql: Config_sql = (&*SQL_TABLES).clone();
    
    let request_authentication_output: Request_authentication_output = match request_authentication(None, params, "/keyword/update", false).await {
        Ok(data) => data,
        Err(e) => return status::Custom(Status::Unauthorized, not_authorized())
    };

    // TODO: There should be an action logic pipeline.

    let metadata_actions = body.actions.clone().unwrap();
    if (metadata_actions.len() == 0) {
        return status::Custom(Status::BadRequest, error_message("body.actions exists but has no items. You must provide at least 1 action."));
    }
    // For now, limit metadata_actions to 10, because they can have a lot of keywords.
    if (metadata_actions.len() >= 10) {
        return status::Custom(Status::BadRequest, error_message("body.actions cannot have more than 10 items."));
    }
    
    for action_data in metadata_actions.iter().clone() {
        // Normallly it would matter what the value of unwrap_or was here, since we're trying to check the original value, in this case checking if it's None, but it doesn't matter here because there is a check for 'create' or 'update'.
        let action_type = action_data.action.clone().unwrap_or(String::new());
        if (action_type != "create" && action_type != "update") {
            return status::Custom(Status::BadRequest, error_message("body.actions.action must be create/update."));
        }
        if (action_type == "update" && action_data.id.is_none() == true) {
            return status::Custom(Status::BadRequest, error_message("body.actions.id must be specified when body.actions.action='update'"));
        }
        if (action_type == "create" && action_data.id.is_none() == false) {
            return status::Custom(Status::BadRequest, error_message("body.actions.id cannot be specified when body.actions.action='create'"));
        }

        // We don't care if body.description is provided, it's not required, but we do want to check the length.
        let description = action_data.description.clone();
        if (description.is_none() == false && description.clone().unwrap().len() > 200000) {
            return status::Custom(Status::BadRequest, error_message("body.actions.description cannot be more than 200000 characters."));
        }
        
        let external_link = action_data.external_link.clone();
        if (external_link.is_none() == false && external_link.clone().unwrap().len() > 4000) {
            return status::Custom(Status::BadRequest, error_message("body.actions.external_link cannot be more than 4000 characters."));
        }
        if (external_link.is_none() == false && Url::parse(&external_link.clone().unwrap()).is_ok() == false) {
            return status::Custom(Status::BadRequest, error_message("body.actions.external_link is an invalid URL."));
        }

        // TODO: If exists, check this is a valid image id.
        let image = action_data.image.clone();

        // TODO: If exists, check this is a valid URL via an image format, and check the URL isn't ridiculously long.
        let external_image = action_data.external_image.clone();
        if (external_image.is_none() == false && external_image.clone().unwrap().len() > 4000) {
            return status::Custom(Status::BadRequest, error_message("body.actions.external_image cannot be more than 4000 characters."));
        }
        if (external_image.is_none() == false && Url::parse(&external_image.clone().unwrap()).is_ok() == false) {
            return status::Custom(Status::BadRequest, error_message("body.actions.external_image is an invalid URL."));
        }

        // TODO: Ensure keyword have no spaces, and is correctly formatted, with a length limit.
        if (action_data.keywords.is_none() == true) {
            return status::Custom(Status::BadRequest, error_message("body.keywords is null or whitespace."));
        }
        let keywords = action_data.keywords.clone().unwrap();

        // TODO: Implement a check, that checks if action_data.keywords + existing keywords is over 100.
        // if (action_data.keywords.len() >= 40) {
        //     return status::Custom(Status::BadRequest, error_message("action_data.keywords cannot have more than 40 items."));
        // }

        let mut keyword_metadata = generate_random_id();
        // TODO: Check keyword_metdata item exists when updating!
        // TODO: WHEN ADDING SPECIFIED KEYWORDS FOR AN ITEM (action_data.item) AND VERIFY THE USER HAS OWNER ACCESS TO IT.

        // Keyword action basic validation checks.
        for keyword_action_data in keywords.iter().clone() {
            let mut keyword_action = keyword_action_data.clone();
            // TODO: There should be an action logic pipeline.
            // Normallly it would matter what the value of unwrap_or was here, since we're trying to check the original value, in this case checking if it's None, but it doesn't matter here because there is a check for 'create' or 'update'.
            let action_type = keyword_action.action.clone().unwrap_or(String::new());
            if (action_type != "create" && action_type != "remove") {
                return status::Custom(Status::BadRequest, error_message("action_data.actions.action must be create/remove."));
            }

            // removed because we always require action.word
            // if (action_type == "update" && is_null_or_whitespace(action.row_id.clone()) == true) {
            //     return status::Custom(Status::BadRequest, error_message("action_data.actions.id must be specified when action_data.keywords.actions.action='update'"));
            // }
            // if (action_type == "create" && is_null_or_whitespace(action.row_id.clone()) == false) {
            //     return status::Custom(Status::BadRequest, error_message("action_data.actions.id cannot be specified when action_data.keywords.actions.action='create'"));
            // }

            if (is_null_or_whitespace(keyword_action.word.clone()) == true) {
                return status::Custom(Status::BadRequest, error_message("action_data.keywords.actions.word is null or whitespace."));
            }

            let word = keyword_action.word.clone().expect("missing action.word").to_lowercase(); // It's important the keyword is lowercase!
            if (word.len() > 1000) {
                return status::Custom(Status::BadRequest, error_message("action_data.keywords.actions.word cannot be longer than 1000 characters."));
            }
        }

        if (action_type == "update") {
            // We know 'body.id' exists, because we checked when validating the 'body.action'.
            keyword_metadata = action_data.id.clone().unwrap(); 

            let (folder, error_to_respond_with) = keyword_get(keyword_metadata.clone(), Some(request_authentication_output.user_id.clone())).await.expect("Failed to get folder.");  

            if (folder.is_none() == true) {
                return status::Custom(Status::BadRequest, error_message(&format!("Keyword metadata does not exist: '{}'", keyword_metadata.clone())));
            }

            sql_query(&format!("UPDATE {} SET description=?, external_link=?, external_image=? WHERE id=? AND owner=?", sql.keyword_metadata.clone().unwrap()))
            .bind::<Nullable<Text>, _>(description.clone())
            .bind::<Nullable<Text>, _>(external_link.clone())
            .bind::<Nullable<Text>, _>(external_image.clone())
            .bind::<Text, _>(keyword_metadata.clone())
            .bind::<Text, _>(request_authentication_output.user_id.clone())
            .execute(&mut db)
            .expect("Something went wrong querying the DB.");
        } else if (action_type == "create") {
            sql_query(&format!("INSERT INTO {} (id, owner, description, external_link, external_image, created) VALUES (?, ?, ?, ?, ?, ?)", sql.keyword_metadata.clone().unwrap()))
            .bind::<Text, _>(keyword_metadata.clone())
            .bind::<Text, _>(request_authentication_output.user_id.clone())
            .bind::<Nullable<Text>, _>(description.clone())
            .bind::<Nullable<Text>, _>(external_link.clone())
            .bind::<Nullable<Text>, _>(external_image.clone())
            .bind::<BigInt, _>(get_epoch())
            .execute(&mut db)
            .expect("Something went wrong querying the DB.");
        }

        // Write data.
        for keyword_action in keywords.iter().clone() {
            let word = keyword_action.word.clone().unwrap();

            let action_type = keyword_action.action.clone().unwrap();
            if (action_type == "create") {
                sql_query(&format!("INSERT INTO {} (keyword, owner, keyword_metadata, created) VALUES (?, ?, ?, ?)", sql.keyword.clone().unwrap()))
                .bind::<Text, _>(word.clone())
                .bind::<Text, _>(request_authentication_output.user_id.clone())
                .bind::<Nullable<Text>, _>(keyword_metadata.clone())
                .bind::<BigInt, _>(get_epoch())
                .execute(&mut db)
                .expect("Something went wrong querying the DB.");
            } else if (action_type == "remove") {
                sql_query(&format!("DELETE FROM {} WHERE keyword=? AND keyword_metadata=? AND owner=?", sql.keyword.clone().unwrap()))
                .bind::<Nullable<Text>, _>(word.clone())
                .bind::<Text, _>(keyword_metadata.clone())
                .bind::<Text, _>(request_authentication_output.user_id.clone())
                .execute(&mut db)
                .expect("Something went wrong querying the DB.");
            }
        }
    }

    // TODO: Check keyword isn't being used elsewhere.

    return status::Custom(Status::Ok, json!({
        "ok": true
    }));
}