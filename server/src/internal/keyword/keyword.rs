use indexmap::IndexMap;

use keyword_metadata::item;
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

pub async fn keyword_list(mut db: Connection<Db>, user_id: String, ids: Option<Vec<String>>, text: Option<String>) -> Result<(Vec<Rendered_keyword>, Option<Value>, Connection<Db>), String> {
    let mut query = crate::tables::keywords::table
    .inner_join(keyword_metadata::table.on(crate::tables::keyword_metadata::id.eq(keywords::keyword_metadata)))
    .filter(keyword_metadata::owner.eq(user_id.clone()))
    .into_boxed();

    if ids.is_none() == false && ids.clone().unwrap().len() > 0 {
        query = query.filter(keyword_metadata::id.eq_any(ids.clone().unwrap()));
    }

    // Diesel doesn't support SQL array functions (like ARRAY_AGG), so we'll have to fetch a bunch of results and filter them out ourselves. It's incredibly annoying, but if we used a raw SQl query, filtering would become riskier, and manually adding an SQL_function to Diesel is a bad idea.

    let results: Vec<(Option<Keyword_metadata>, Option<String>)> = query
    .select((
        keyword_metadata::all_columns.nullable(),
        crate::tables::keywords::keyword.nullable(),
    ))
    .order(keyword_metadata::created.desc())
    .load::<(
        Option<Keyword_metadata>,
        Option<String>
    )>(&mut db)
    .await
    .expect("Something went wrong querying the DB.");

    let mut metadata_store: IndexMap<String, Rendered_keyword> = IndexMap::new();
    for (metadata_wrapped, keyword) in results {
        if metadata_wrapped.is_none() == true || keyword.is_none() == true {
            continue;
        }

        if (text.clone().is_none() == false) {
            // Skip irrelevant keyword(s) (a keyword that isn't used within the provided text).
            let text_unwrap = text.clone().unwrap().to_lowercase();
            let keyword_unwrap = keyword.clone().clone().unwrap().to_lowercase();
            if (text_unwrap.clone().contains(&format!(" {} ", &keyword_unwrap.clone())) == false && text_unwrap.clone().trim() != keyword_unwrap.clone()) {
                continue;
            }
        }

        let metadata = metadata_wrapped.unwrap();

        if let Some(new_metadata) = metadata_store.get_mut(&metadata.id.clone()) {
            let mut new_keywords = new_metadata.keywords.clone().unwrap();
            new_keywords.push(keyword.unwrap());
            new_metadata.keywords = Some(new_keywords);
        } else {
            metadata_store.insert(metadata.id.clone(), Rendered_keyword {
                id: metadata.id.clone(),
                owner: metadata.owner,
                description: metadata.description,
                external_link: metadata.external_link,
                image: metadata.image,
                external_image: metadata.external_image,
                item: metadata.item,
                created: metadata.created,
                keywords: Some(vec![keyword.unwrap()]),
            });
        }
    }

    let mut item_results: Vec<Rendered_keyword> = metadata_store.values().cloned().collect();

    return Ok((
        item_results,
        None,
        db
    ));
}

// TODO: KEYWORD_GET NEEDS TO BE CHANGED BECAUSE IT INCLUDES RAW DB INFORMATION.
pub async fn keyword_get(mut db: Connection<Db>, id: String, write_authorized: Option<String>) -> Result<(Option<Keyword_sql>, Option<Value>, Connection<Db>), String> {
    let sql: Config_sql = (&*SQL_TABLES).clone();

    let keyword_table = sql.keyword.unwrap();
    let keyword_metadata = sql.keyword_metadata.unwrap();

    // TODO: Keywords shouldn't have owners (and keyword_metadata and keyword join might be the wrong way around??)
    let selection = format!("SELECT keyword_metadata.id AS id, GROUP_CONCAT(keyword.keyword) AS keywords, keyword_metadata.description, keyword_metadata.image, keyword_metadata.external_image, keyword_metadata.external_link, keyword_metadata.owner, keyword_metadata.created
FROM {}
INNER JOIN {}
ON keyword.owner = keyword_metadata.owner", keyword_table.clone(), keyword_metadata.clone());

    let mut results: Vec<Keyword_sql> = Vec::new();

    results = sql_query(&format!("{} WHERE keyword_metadata.id=?", selection))
    .bind::<Text, _>(id.clone())
    .load::<Keyword_sql>(&mut db)
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

pub async fn keywords_from_text(mut db: Connection<Db>, text: String, user_id: String) -> Result<(Vec<Rendered_keyword>, Option<Value>, Connection<Db>), String> {
    let (item_results, error_to_respond_with, keyword_db) = keyword_list(db, user_id, None, Some(text)).await.expect("Failed to get keyword_list");
    db = keyword_db;

    return Ok((
        item_results,
        None,
        db
    ));
}