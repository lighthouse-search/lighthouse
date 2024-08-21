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

pub async fn content_list(mut db: Connection<Db>, item: String, user_id: String) -> Result<(Vec<Mindmap_item_content>, Option<Value>, Connection<Db>), String> {
    let sql: Config_sql = (&*SQL_TABLES).clone();

    let folder_table = sql.folder.unwrap();
    let item_table = sql.item.unwrap();
    let item_content_table = sql.item_content.unwrap();

    // not needed and handled by item_get, but just in-case I need it later: LEFT JOIN item AS item_table ON item_table.id=item_content.item
    let selection = format!("SELECT parent, item, rank, content, row_id, created FROM {}", item_content_table.clone());

    // Check we have permission to access the folder.
    let (item_status, error_to_respond_with, item_db) = crate::internal::item::index::item_get(db, item.clone(), Some(user_id.clone())).await.expect("Error looking up folder.");
    db = item_db;

    if (error_to_respond_with.is_none() == false) {
        return Ok((Vec::new(), Some(error_to_respond_with.unwrap()), db));
    }
    if (item_status.is_none() == true) {
        return Ok((Vec::new(), Some(error_message("Item was not found.")), db));
    }

    // User ID is checked in the item_get above. It hasn't been missed!

    let mut item_content_results: Vec<Mindmap_item_content> = sql_query(&format!("{} WHERE item=? ORDER BY created ASC", selection))
    .bind::<Text, _>(item.clone())
    .load::<Mindmap_item_content>(&mut db)
    .await
    .expect("Something went wrong querying the DB.");

    return Ok((
        item_content_results,
        None,
        db
    ));
}

// pub async fn content_get(mut db: Connection<Db>, id: String, write_authorized: Option<String>) -> Result<(Option<Mindmap_item>, Option<Value>, Connection<Db>), String> {
//     let sql: Config_sql = (&*SQL_TABLES).clone();

//     let results: Vec<Mindmap_item> = sql_query(format!("SELECT id, title, owner, created, visibility FROM {} WHERE owner=? ORDER BY created DESC", sql.item.unwrap()))
//     .bind::<Text, _>(id.clone())
//     .load::<Mindmap_item,>(&mut db)
//     .await
//     .expect("Something went wrong querying the DB.");

//     if (results.len() == 0) {
//         // Not found.
//         return Ok((None, None, db));
//     }

//     let result = results[0].clone();

//     if (write_authorized.is_none() == false) {
//         if (result.owner.clone().unwrap() != write_authorized.unwrap()) {
//             // We found a result, but the user doesn't have write access. [Currently we're just checking if they're the owner until there is a better system]
//             return Ok((None, Some(error_message("You need write authorization for the specified folder.")), db));
//         }
//     }

//     return Ok((
//         Some(result),
//         None,
//         db
//     ));
// }

// pub async fn content_update(mut db: Connection<Db>, item: String, user_id: String) -> Result<(Vec<Mindmap_item_content>, Option<Value>, Connection<Db>), String> {
//     let sql: Config_sql = (&*SQL_TABLES).clone();

//     let folder_table = sql.folder.unwrap();
//     let item_table = sql.item.unwrap();
//     let item_content_table = sql.item_content.unwrap();

//     // not needed and handled by item_get, but just in-case I need it later: LEFT JOIN item AS item_table ON item_table.id=item_content.item
//     let selection = format!("UPDATE {} SET parent=?, item=?, rank=?, content=?, row_id=?, created=? WHERE row_id=?", item_content_table.clone());

//     // Check we have permission to access the folder.
//     let (item_status, error_to_respond_with, item_db) = crate::internal::item::index::item_get(db, item.clone(), Some(user_id.clone())).await.expect("Error looking up folder.");
//     db = item_db;

//     if (error_to_respond_with.is_none() == false) {
//         return Ok((Vec::new(), Some(error_to_respond_with.unwrap()), db));
//     }
//     if (item_status.is_none() == true) {
//         return Ok((Vec::new(), Some(error_message("Item was not found.")), db));
//     }

//     // User ID is checked in the item_get above. It hasn't been missed!

//     let mut item_content_results: Vec<Mindmap_item_content> = sql_query(&format!("{} WHERE item=? ORDER BY created DESC", selection))
//     .bind::<Text, _>(item.clone())
//     .load::<Mindmap_item_content>(&mut db)
//     .await
//     .expect("Something went wrong querying the DB.");

//     return Ok((
//         item_content_results,
//         None,
//         db
//     ));
// }