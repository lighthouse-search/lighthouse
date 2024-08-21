use rocket::serde::{Serialize, Deserialize, json::Json};
use rocket::serde::json::{Value, json};
use rocket::response::{status, status::Custom};
use rocket::http::Status;

use rocket_db_pools::{Database, Connection};
use rocket_db_pools::diesel::{MysqlPool, prelude::*};
use diesel::sql_query;
use diesel::sql_types::*;

use crate::global::{generate_random_id, get_epoch, is_null_or_whitespace, request_authentication};
use crate::responses::*;
use crate::structs::*;
use crate::tables::*;
use crate::SQL_TABLES;

pub async fn folder_list(mut db: Connection<Db>, user_id: String, folder: Option<String>) -> Result<(Vec<Mindmap_folder>, Option<Value>, Connection<Db>), String> {
    let sql: Config_sql = (&*SQL_TABLES).clone();

    // format!() is not for values. It uses heavily vetted and sanitized values that are directly from the admin's configuration on local environment variables.
    let selection = format!("SELECT id, title, owner, created, visibility, folder FROM {}", sql.folder.unwrap());

    let mut folder_results: Vec<Mindmap_folder> = Vec::new();
    if (is_null_or_whitespace(folder.clone()) == false) {
        // Check we have permission to access the folder.
        let folder_id = folder.unwrap();
        let (folder_status, error_to_respond_with, folder_db) = folder_get(db, folder_id.clone(), Some(user_id.clone())).await.expect("Error looking up folder.");
        db = folder_db;

        if (error_to_respond_with.is_none() == false) {
            return Ok((Vec::new(), Some(error_to_respond_with.unwrap()), db));
        }
        if (folder_status.is_none() == true) {
            return Ok((Vec::new(), Some(error_message("params.folder was not found, or you are not authorized to access it.")), db));
        }

        // format!() is not for values. It uses heavily vetted and sanitized values that are directly from the admin's configuration on local environment variables.
        folder_results = sql_query(&format!("{} WHERE owner=? AND folder=? ORDER BY created DESC", selection))
        .bind::<Text, _>(user_id.clone())
        .bind::<Text, _>(folder_id.clone())
        .load::<Mindmap_folder>(&mut db)
        .await
        .expect("Something went wrong querying the DB.");
    } else {
        // format!() is not for values. It uses heavily vetted and sanitized values that are directly from the admin's configuration on local environment variables.
        folder_results = sql_query(&format!("{} WHERE owner=? AND folder IS NULL ORDER BY created DESC", selection))
        .bind::<Text, _>(user_id.clone())
        .load::<Mindmap_folder>(&mut db)
        .await
        .expect("Something went wrong querying the DB.");
    }

    return Ok((
        folder_results,
        None,
        db
    ));
}

pub async fn folder_get(mut db: Connection<Db>, id: String, read_authorized: Option<String>) -> Result<(Option<Mindmap_folder>, Option<Value>, Connection<Db>), String> {
    let sql: Config_sql = (&*SQL_TABLES).clone();

    // format!() is not for values. It uses heavily vetted and sanitized values that are directly from the admin's configuration on local environment variables.
    let results: Vec<Mindmap_folder> = sql_query(format!("SELECT id, title, owner, created, visibility FROM {} WHERE id=? ORDER BY created DESC", sql.folder.unwrap()))
    .bind::<Text, _>(id.clone())
    .load::<Mindmap_folder>(&mut db)
    .await
    .expect("Something went wrong querying the DB.");

    if (results.len() == 0) {
        // Not found.
        return Ok((None, None, db));
    }

    let result = results[0].clone();

    if (read_authorized.is_none() == false) {
        if (result.owner.clone().unwrap() != read_authorized.unwrap()) {
            // We found a result, but the user doesn't have read access. [Currently we're just checking if they're the owner until there is a better system]
            return Ok((None, Some(error_message("You need read authorization for the specified folder.")), db));
        }
    }

    return Ok((
        Some(result),
        None,
        db
    ));
}