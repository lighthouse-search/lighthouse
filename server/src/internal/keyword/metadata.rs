use rocket::serde::{Serialize, Deserialize, json::Json};
use rocket::serde::json::{Value, json};
use rocket::response::{status, status::Custom};
use rocket::http::Status;

use rocket_db_pools::{Database, Connection};
use rocket_db_pools::diesel::{MysqlPool, prelude::*};
use diesel::sql_query;
use diesel::sql_types::*;

use crate::global::{generate_random_id, get_epoch, is_null_or_whitespace, request_authentication};
use crate::internal::item::index::item_get;
use crate::responses::*;
use crate::structs::*;
use crate::tables::*;
use crate::SQL_TABLES;

pub async fn keyword_metadata_list(mut db: Connection<Db>, user_id: String, item: Option<String>) -> Result<(Vec<Mindmap_item>, Option<Value>, Connection<Db>), String> {
    let sql: Config_sql = (&*SQL_TABLES).clone();

    let keyword_metadata_table = sql.keyword_metadata.unwrap();

    let selection = format!("SELECT id, image, owner, item, created FROM {}", keyword_metadata_table.clone());

    let mut item_results: Vec<Mindmap_item> = Vec::new();
    if (is_null_or_whitespace(item.clone()) == false) {
        // Check we have permission to access the document.
        let item_id = item.unwrap();
        let (item_status, error_to_respond_with, item_db) = item_get(db, item_id.clone(), Some(user_id.clone())).await.expect("Error looking up item.");
        db = item_db;

        if (error_to_respond_with.is_none() == false) {
            return Ok((Vec::new(), Some(error_to_respond_with.unwrap()), db));
        }
        if (item_status.is_none() == true) {
            return Ok((Vec::new(), Some(error_message("params.item was not found, or you are not authorized to access it.")), db));
        }

        item_results = sql_query(&format!("{} WHERE owner=? AND item=? ORDER BY created DESC", selection))
        .bind::<Text, _>(user_id.clone())
        .bind::<Nullable<Text>, _>(item_id.clone())
        .load::<Mindmap_item>(&mut db)
        .await
        .expect("Something went wrong querying the DB.");
    } else {
        item_results = sql_query(&format!("{} WHERE owner=? ORDER BY created DESC", selection))
        .bind::<Text, _>(user_id.clone())
        .load::<Mindmap_item>(&mut db)
        .await
        .expect("Something went wrong querying the DB.");
    }
    
    return Ok((
        item_results,
        None,
        db
    ));
}

pub async fn keyword_metadata_get(mut db: Connection<Db>, id: String, write_authorized: Option<String>) -> Result<(Option<Mindmap_item>, Option<Value>, Connection<Db>), String> {
    let sql: Config_sql = (&*SQL_TABLES).clone();

    let results: Vec<Mindmap_item> = sql_query(format!("SELECT id, image, owner, item, created FROM {} WHERE id=? AND owner=? ORDER BY created DESC", sql.item.unwrap()))
    .bind::<Text, _>(id.clone())
    .bind::<Text, _>(write_authorized.clone().unwrap()) // temp until permissions system
    .load::<Mindmap_item,>(&mut db)
    .await
    .expect("Something went wrong querying the DB.");

    if (results.len() == 0) {
        // Not found.
        return Ok((None, None, db));
    }

    let result = results[0].clone();

    if (write_authorized.is_none() == false) {
        if (result.owner.clone().unwrap() != write_authorized.unwrap()) {
            // We found a result, but the user doesn't have write access. [Currently we're just checking if they're the owner until there is a better system]
            return Ok((None, Some(error_message("You need write authorization for the specified folder.")), db));
        }
    }

    return Ok((
        Some(result),
        None,
        db
    ));
}