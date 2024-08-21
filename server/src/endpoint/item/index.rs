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
use crate::responses::*;
use crate::structs::*;
use crate::tables::*;
use crate::SQL_TABLES;

#[get("/list?<folder>")]
pub async fn item_list(mut db: Connection<Db>, folder: Option<String>, params: &Query_string) -> Custom<Value> {
    let sql: Config_sql = (&*SQL_TABLES).clone();

    let folder_table = sql.folder.unwrap();
    let item_table = sql.item.unwrap();

    let request_authentication_output: Request_authentication_output = match request_authentication(db, None, params, "/item/list", false).await {
        Ok(data) => data,
        Err(e) => return status::Custom(Status::Unauthorized, not_authorized())
    };
    db = request_authentication_output.returned_connection;

    let mut folder_data_public: Option<Mindmap_folder_public> = None;
    if (is_null_or_whitespace(folder.clone()) == false) {
        let (folder_status, error_to_respond_with, folder_db) = folder_get(db, folder.clone().unwrap(), Some(request_authentication_output.user_id.clone())).await.expect("Error looking up folder.");
        db = folder_db;

        if (error_to_respond_with.is_none() == false) {
            return status::Custom(Status::BadRequest, error_to_respond_with.unwrap());
        }
        if (folder_status.is_none() == true) {
            return status::Custom(Status::NotFound, error_message("body.folder was not found, or you are not authorized to access it."));
        }

        let folder_data: Mindmap_folder = folder_status.unwrap();
        folder_data_public = Some(folder_data.into());
    }

    let (folder_result, error_to_respond_with, folder_db) = folder_list(db, request_authentication_output.user_id.clone(), folder.clone()).await.expect("Failed to list folders");
    db = folder_db;

    let (item_result, error_to_respond_with, item_db) = crate::internal::item::index::item_list(db, request_authentication_output.user_id.clone(), folder.clone()).await.expect("Failed to list items");
    db = item_db;

    let mut folder_result_public: Vec<Mindmap_folder_public> = folder_result
    .into_iter()
    .map(Mindmap_folder_public::from)
    .collect();

    let mut item_result_public: Vec<Mindmap_item_public> = item_result
    .into_iter()
    .map(Mindmap_item_public::from)
    .collect();

    let mut data: Vec<Value> = Vec::new();

    for element in folder_result_public.into_iter() {
        let mut output = serde_json::to_value(&element).expect("Failed to serialize");

        if let Some(obj) = output.as_object_mut() {
            obj.insert("type".to_string(), Value::from("folder".to_string()));
        } else {
            println!("The JSON value is not an object.");
        }

        data.push(output);
    }

    for element in item_result_public.into_iter() {
        let mut output = serde_json::to_value(&element).expect("Failed to serialize");

        if let Some(obj) = output.as_object_mut() {
            obj.insert("type".to_string(), Value::from("item".to_string()));
        } else {
            println!("The JSON value is not an object.");
        }

        data.push(output);
    }

    data.sort_by(|a, b| {
        let mut created_a = 0;
        let mut created_b = 0;

        if let Some(number) = a.get("created").and_then(Value::as_i64) {
            created_a = number;
        }

        if let Some(number) = b.get("created").and_then(Value::as_i64) {
            created_b = number;
        }

        created_b.cmp(&created_a)
    });

    status::Custom(Status::Ok, json!({
        "ok": true,
        "data": data,
        "folder": folder_data_public
    }))
}

#[post("/update", format = "application/json", data = "<body>")]
pub async fn item_update(mut db: Connection<Db>, params: &Query_string, mut body: Json<Item_update_body>) -> Custom<Value> {
    let sql: Config_sql = (&*SQL_TABLES).clone();

    let folder_table = sql.folder.unwrap();
    let item_table = sql.item.unwrap();
    
    let request_authentication_output: Request_authentication_output = match request_authentication(db, None, params, "/item/update", false).await {
        Ok(data) => data,
        Err(e) => return status::Custom(Status::Unauthorized, not_authorized())
    };
    db = request_authentication_output.returned_connection;

    // TODO: There should be an action logic pipeline.
    
    // Normallly it would matter what the value of unwrap_or was here, since we're trying to check the original value, in this case checking if it's None, but it doesn't matter here because there is a check for 'create' or 'update'.
    let action = body.action.clone().unwrap_or(String::new());
    if (action != "create" && action != "update") {
        return status::Custom(Status::BadRequest, error_message("body.action must be create/update."));
    }
    if (action == "update" && body.id.is_none() == true) {
        return status::Custom(Status::BadRequest, error_message("body.id must be specified when body.action='update'"));
    }
    if (action == "create" && body.id.is_none() == false) {
        return status::Custom(Status::BadRequest, error_message("body.id cannot be specified when body.action='create'"));
    }
    if (action == "create" && body.folder.is_none() == true) {
        return status::Custom(Status::BadRequest, error_message("body.folder is null or whitespace."));
    }

    if (body.title.is_none() == true) {
        return status::Custom(Status::BadRequest, error_message("body.title is null or whitespace."));
    }

    let title = body.title.clone().expect("missing body.title");

    let folder_id: Option<String> = body.folder.clone();
    if (folder_id.is_none() == false) {
        let (folder_status, error_to_respond_with, folder_db) = folder_get(db, folder_id.clone().unwrap(), Some(request_authentication_output.user_id.clone())).await.expect("Error looking up folder.");
        db = folder_db;

        if (error_to_respond_with.is_none() == false) {
            return status::Custom(Status::BadRequest, error_to_respond_with.unwrap());
        }
        if (folder_status.is_none() == true) {
            return status::Custom(Status::NotFound, error_message("body.folder was not found, or you are not authorized to access it."));
        }
    }

    let visibilities = vec!["public", "unlisted", "private"];
    let visibility = body.visibility.clone().unwrap_or(String::new());
    if (visibility.contains(&visibility.to_string()) == false) {
        return status::Custom(Status::BadRequest, error_message(&format!("body.visibility is invalid, must be {}", visibilities.join("/"))));
    }

    let mut item_id = generate_random_id();
    // let number: i32 = rand::thread_rng().gen_range(0..999999);

    if (action == "update") {
        // We know 'body.id' exists, because we checked when validating the 'body.action'.
        item_id = body.id.clone().unwrap(); 

        let (item, error_to_respond_with, item_db) = item_get(db, item_id.clone(), Some(request_authentication_output.user_id.clone())).await.expect("Failed to get folder.");  
        db = item_db;
        
        if (item.is_none() == true) {
            return status::Custom(Status::BadRequest, error_message(&format!("Item does not exist: '{}'", item_id.clone())));
        }

        // format!() is not for values. It uses heavily vetted and sanitized values that are directly from the admin's configuration on local environment variables.
        let result: Vec<Mindmap_item> = sql_query(&format!("UPDATE {} SET title=?, folder=?, visibility=? WHERE id=? AND owner=?", item_table))
        .bind::<Text, _>(title.clone())
        .bind::<Nullable<Text>, _>(folder_id.clone())
        .bind::<Text, _>(visibility.clone())
        .bind::<Text, _>(item_id.clone())
        .bind::<Text, _>(request_authentication_output.user_id.clone())
        .load::<Mindmap_item>(&mut db)
        .await
        .expect("Something went wrong querying the DB.");
    } else if (action == "create") {
        // format!() is not for values. It uses heavily vetted and sanitized values that are directly from the admin's configuration on local environment variables.
        let result: Vec<Mindmap_item> = sql_query(&format!("INSERT INTO {} (id, title, folder, visibility, owner, created) VALUES (?, ?, ?, ?, ?, ?)", item_table))
        .bind::<Text, _>(item_id.clone())
        .bind::<Text, _>(title.clone())
        .bind::<Nullable<Text>, _>(folder_id.clone())
        .bind::<Text, _>(visibility.clone())
        .bind::<Text, _>(request_authentication_output.user_id.clone())
        .bind::<BigInt, _>(get_epoch())
        .load::<Mindmap_item>(&mut db)
        .await
        .expect("Something went wrong querying the DB.");
    }

    return status::Custom(Status::Ok, json!({
        "ok": true,
        "item_id": item_id.clone()
    }));
}

#[post("/content/update", format = "application/json", data = "<body>")]
pub async fn item_content_update(mut db: Connection<Db>, params: &Query_string, mut body: Json<Item_update_body>) -> Custom<Value> {
    let sql: Config_sql = (&*SQL_TABLES).clone();

    let folder_table = sql.folder.unwrap();
    let item_table = sql.item.unwrap();
    
    let request_authentication_output: Request_authentication_output = match request_authentication(db, None, params, "/item/update", false).await {
        Ok(data) => data,
        Err(e) => return status::Custom(Status::Unauthorized, not_authorized())
    };
    db = request_authentication_output.returned_connection;

    // TODO: There should be an action logic pipeline.
    
    // Normallly it would matter what the value of unwrap_or was here, since we're trying to check the original value, in this case checking if it's None, but it doesn't matter here because there is a check for 'create' or 'update'.
    let action = body.action.clone().unwrap_or(String::new());
    if (action != "create" && action != "update") {
        return status::Custom(Status::BadRequest, error_message("body.action must be create/update."));
    }
    if (action == "update" && body.id.is_none() == true) {
        return status::Custom(Status::BadRequest, error_message("body.id must be specified when body.action='update'"));
    }
    if (action == "create" && body.id.is_none() == false) {
        return status::Custom(Status::BadRequest, error_message("body.id cannot be specified when body.action='create'"));
    }
    if (action == "create" && body.folder.is_none() == true) {
        return status::Custom(Status::BadRequest, error_message("body.folder is null or whitespace."));
    }

    if (body.title.is_none() == true) {
        return status::Custom(Status::BadRequest, error_message("body.title is null or whitespace."));
    }

    let title = body.title.clone().expect("missing body.title");

    let folder_id: Option<String> = body.folder.clone();
    if (folder_id.is_none() == false) {
        let (folder_status, error_to_respond_with, folder_db) = folder_get(db, folder_id.clone().unwrap(), Some(request_authentication_output.user_id.clone())).await.expect("Error looking up folder.");
        db = folder_db;

        if (error_to_respond_with.is_none() == false) {
            return status::Custom(Status::BadRequest, error_to_respond_with.unwrap());
        }
        if (folder_status.is_none() == true) {
            return status::Custom(Status::NotFound, error_message("body.folder was not found, or you are not authorized to access it."));
        }
    }

    let visibilities = vec!["public", "unlisted", "private"];
    let visibility = body.visibility.clone().unwrap_or(String::new());
    if (visibility.contains(&visibility.to_string()) == false) {
        return status::Custom(Status::BadRequest, error_message(&format!("body.visibility is invalid, must be {}", visibilities.join("/"))));
    }

    let mut item_id = generate_random_id();
    // let number: i32 = rand::thread_rng().gen_range(0..999999);

    if (action == "update") {
        // We know 'body.id' exists, because we checked when validating the 'body.action'.
        item_id = body.id.clone().unwrap(); 

        let (item, error_to_respond_with, item_db) = item_get(db, item_id.clone(), Some(request_authentication_output.user_id.clone())).await.expect("Failed to get folder.");  
        db = item_db;
        
        if (item.is_none() == true) {
            return status::Custom(Status::BadRequest, error_message(&format!("Item does not exist: '{}'", item_id.clone())));
        }

        // format!() is not for values. It uses heavily vetted and sanitized values that are directly from the admin's configuration on local environment variables.
        let result: Vec<Mindmap_item> = sql_query(&format!("UPDATE {} SET title=?, folder=?, visibility=? WHERE id=? AND owner=?", item_table))
        .bind::<Text, _>(title.clone())
        .bind::<Nullable<Text>, _>(folder_id.clone())
        .bind::<Text, _>(visibility.clone())
        .bind::<Text, _>(item_id.clone())
        .bind::<Text, _>(request_authentication_output.user_id.clone())
        .load::<Mindmap_item>(&mut db)
        .await
        .expect("Something went wrong querying the DB.");
    } else if (action == "create") {
        // format!() is not for values. It uses heavily vetted and sanitized values that are directly from the admin's configuration on local environment variables.
        let result: Vec<Mindmap_item> = sql_query(&format!("INSERT INTO {} (id, title, folder, visibility, owner, created) VALUES (?, ?, ?, ?, ?, ?)", item_table))
        .bind::<Text, _>(item_id.clone())
        .bind::<Text, _>(title.clone())
        .bind::<Nullable<Text>, _>(folder_id.clone())
        .bind::<Text, _>(visibility.clone())
        .bind::<Text, _>(request_authentication_output.user_id.clone())
        .bind::<BigInt, _>(get_epoch())
        .load::<Mindmap_item>(&mut db)
        .await
        .expect("Something went wrong querying the DB.");
    }

    return status::Custom(Status::Ok, json!({
        "ok": true,
        "item_id": item_id.clone()
    }));
}