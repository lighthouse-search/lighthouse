use rocket::serde::{Serialize, Deserialize, json::Json};
use rocket::serde::json::{Value, json};
use rocket::response::{status, status::Custom};
use rocket::http::Status;

use diesel::sql_query;
use diesel::prelude::*;
use diesel::sql_types::*;

use crate::global::{generate_random_id, get_epoch, request_authentication};
use crate::internal::folder::folder_get;
use crate::responses::*;
use crate::structs::*;
use crate::tables::*;
use crate::SQL_TABLES;

#[get("/list")]
pub async fn folder_list(params: &Query_string) -> Custom<Value> {
    let mut db = crate::DB_POOL.get().expect("Failed to get a connection from the pool.");
    let sql: Config_sql = (&*SQL_TABLES).clone();

    let request_authentication_output: Request_authentication_output = match request_authentication(None, params, "/folder/list", false).await {
        Ok(data) => data,
        Err(e) => return status::Custom(Status::Unauthorized, not_authorized())
    };

    // format!() is not for values. It uses heavily vetted and sanitized values that are directly from the admin's configuration on local environment variables.
    let folder_result: Vec<Mindmap_folder> = sql_query(format!("SELECT id, title, owner, created, visibility FROM {} WHERE owner=? ORDER BY created DESC", sql.folder.unwrap()))
    .bind::<Text, _>(request_authentication_output.user_id)
    .load::<Mindmap_folder>(&mut db)
    .expect("Something went wrong querying the DB.");

    let mut folder_public: Vec<Mindmap_folder_public> = folder_result
    .into_iter()
    .map(Mindmap_folder_public::from)
    .collect();

    status::Custom(Status::Ok, json!({
        "ok": true,
        "data": folder_public
    }))
}

#[post("/update", format = "application/json", data = "<body>")]
pub async fn folder_update(params: &Query_string, mut body: Json<Folder_update_body>) -> Custom<Value> {
    let mut db = crate::DB_POOL.get().expect("Failed to get a connection from the pool.");
    let sql: Config_sql = (&*SQL_TABLES).clone();
    
    let request_authentication_output: Request_authentication_output = match request_authentication(None, params, "/folder/update", false).await {
        Ok(data) => data,
        Err(e) => return status::Custom(Status::Unauthorized, not_authorized())
    };

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

    if (body.title.is_none() == true) {
        return status::Custom(Status::BadRequest, error_message("body.title is null or whitespace."));
    }

    let visibilities = vec!["public", "unlisted", "private"];
    let visibility = body.visibility.clone().unwrap_or(String::new());
    if (visibility.contains(&visibility.to_string()) == false) {
        return status::Custom(Status::BadRequest, error_message(&format!("body.visibility is invalid, must be {}", visibilities.join("/"))));
    }

    if (body.inner_folder.is_none() == false) {
        let folder_id = body.inner_folder.clone().unwrap();
        let (folder_status, error_to_respond_with) = folder_get(folder_id.clone(), Some(request_authentication_output.user_id.clone())).await.expect("Error looking up folder.");

        if (error_to_respond_with.is_none() == false) {
            return status::Custom(Status::BadRequest, error_to_respond_with.unwrap());
        }
        if (folder_status.is_none() == true) {
            return status::Custom(Status::BadRequest, error_message("body.inner_folder was not found, or you are not authorized to access it."));
        }
    }
    let inner_folder = body.inner_folder.clone();

    let title = body.title.clone().expect("missing body.title");

    let mut folder_id = generate_random_id();
    // let number: i32 = rand::thread_rng().gen_range(0..999999);

    if (action == "update") {
        // We know 'body.id' exists, because we checked when validating the 'body.action'.
        folder_id = body.id.clone().unwrap(); 

        let (folder, error_to_respond_with) = folder_get(folder_id.clone(), Some(request_authentication_output.user_id.clone())).await.expect("Failed to get folder.");  

        if (folder.is_none() == true) {
            return status::Custom(Status::BadRequest, error_message(&format!("Folder does not exist: '{}'", folder_id.clone())));
        }

        let result: Vec<Mindmap_folder> = sql_query(&format!("UPDATE {} SET title=?, folder=?, visibility=? WHERE id=? AND owner=?", sql.folder.unwrap()))
        .bind::<Text, _>(title.clone())
        .bind::<Nullable<Text>, _>(inner_folder.clone())
        .bind::<Text, _>(visibility.clone())
        .bind::<Text, _>(folder_id.clone())
        .bind::<Text, _>(request_authentication_output.user_id.clone())
        .load::<Mindmap_folder>(&mut db)
        .expect("Something went wrong querying the DB.");
    } else if (action == "create") {
        let result = sql_query(&format!("INSERT INTO {} (id, title, folder, visibility, owner, created) VALUES (?, ?, ?, ?, ?, ?)", sql.folder.unwrap()))
        .bind::<Text, _>(folder_id.clone())
        .bind::<Text, _>(title.clone())
        .bind::<Nullable<Text>, _>(inner_folder.clone())
        .bind::<Text, _>(visibility.clone())
        .bind::<Text, _>(request_authentication_output.user_id.clone())
        .bind::<BigInt, _>(get_epoch())
        .execute(&mut db)
        .expect("Something went wrong querying the DB.");
    }

    return status::Custom(Status::Ok, json!({
        "ok": true,
        "folder_id": folder_id.clone()
    }));
}