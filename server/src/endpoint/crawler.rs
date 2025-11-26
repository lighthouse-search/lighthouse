use std::vec;

use rocket::response::{Debug, status::Created};
use rocket::response::status;
use rocket::http::Status;
use rocket::response::status::Custom;
use rocket::serde::json::Json;
use rocket::{get, post};

use serde::{Serialize, Deserialize};
use serde_json::{Value, json};

use diesel::prelude::*;
use diesel::sql_types::*;
use diesel::sql_query;

use crate::global::{ generate_random_id, get_timestamp, is_null_or_whitespace, request_authentication };
use crate::responses::*;
use crate::structs::*;
use crate::tables::*;
use crate::ES;

use elasticsearch::{BulkOperation, BulkParts, SearchParts};
use uuid::Uuid;
use url::Url;

#[post("/index", format = "application/json", data = "<body>")]
pub async fn crawler_index(params: &Query_string, mut body: Json<Crawler_index_body>) -> Custom<Value> {
    let mut db = crate::DB_POOL.get().expect("Failed to get a connection from the pool.");

    // // TODO: This authentication needs to be locked to crawlers only.
    // let request_authentication_output: Request_authentication_output = match request_authentication(None, params, "/crawler/index").await {
    //     Ok(data) => data,
    //     Err(e) => return status::Custom(Status::Unauthorized, not_authorized())
    // };

    let actions = body.actions.clone().unwrap();

    // TODO: Return errors here.
    for data in actions.clone() {
        if (is_null_or_whitespace(data.url.clone())) {
            
        }

        if (data.content.is_none() == true) {
            
        }
        let content_unwrapped: Crawler_index_body_action_content = data.content.clone().expect("missing body.content");
        if (is_null_or_whitespace(content_unwrapped.title.clone()) == true) {

        }
        if (is_null_or_whitespace(content_unwrapped.text.clone()) == true) {

        }
        if (content_unwrapped.urls.is_none() == true || content_unwrapped.urls.clone().unwrap().len() == 0) {

        }
    };

    let mut ops: Vec<BulkOperation<Value>> = Vec::with_capacity(actions.len());

    let mut url_queue: Vec<UrlQueue> = Vec::new();

    for data in actions.clone() {
        let url: String = data.url.clone().expect("missing body.url");
        let content: Crawler_index_body_action_content = data.content.clone().expect("missing body.content");

        let created: i64 = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_millis().try_into().unwrap();

        let parsed_url = Url::parse(&url).expect("Failed to parse URL");

        for (_, link) in content.urls.clone().unwrap_or(Vec::new()).iter().enumerate() {
            if (url_queue.iter().position(|x| &x.url == link).is_some()) {
                continue;
            }
            url_queue.push(UrlQueue {
                url: link.clone(),
                referrer: url.clone(),
            });
        }

        ops.push(
            BulkOperation::delete(url.clone()).into()
        );
    
        ops.push(
            BulkOperation::create(json!({
                "url": url.clone(),
                "host": parsed_url.host_str(),
                "content": content,
                "indexed": created
            })
        )
        .id(url)
        .into());
    };

    // Add bulk event data to elasticsearch.
    ES
    .bulk(BulkParts::Index("lighthouse-index"))
    .body(ops)
    .send()
    .await.expect("Failed to insert data.");

    let existing_urls = crate::tables::crawler_queue::table
    .filter(crate::tables::crawler_queue::url.eq_any(
        url_queue.iter().map(|x| x.url.clone()).collect::<Vec<String>>()
    ))
    .load::<Crawler_queue>(&mut *db)
    .expect("Error loading records");

    for (_, item) in existing_urls.iter().enumerate() {
        if let Some(pos) = url_queue.iter().position(|x| *x.url == item.url.clone().unwrap()) {
            url_queue.remove(pos);
        }
    }

    for (_, data) in url_queue.iter().enumerate() {
        diesel::insert_into(crate::tables::crawler_queue::table)
        .values((
            crate::tables::crawler_queue::url.eq(data.url.clone()),
            crate::tables::crawler_queue::referrer.eq(data.referrer.clone()),
            crate::tables::crawler_queue::status.eq("considering"),
            crate::tables::crawler_queue::created.eq(get_timestamp() as i64),
        ))
        .execute(&mut *db).expect("Failed to insert URL.");
    }

    return status::Custom(Status::Ok, json!({
        "ok": true
    }));
}

#[get("/queue")]
pub async fn crawler_queue(params: &Query_string) -> Custom<Value> {
    let mut db = crate::DB_POOL.get().expect("Failed to get a connection from the pool.");

    let queue_result: Option<Crawler_queue> = crate::tables::crawler_queue::table
    .select(
        crate::tables::crawler_queue::all_columns,
    )
    .filter(crate::tables::crawler_queue::status.eq("pending"))
    .first::<Crawler_queue>(&mut *db)
    .optional()
    .expect("Something went wrong querying the DB.");

    if queue_result.is_none() {
        return status::Custom(Status::Ok, json!({
            "ok": true,
            "data": Vec::<Crawler_queue>::new()
        }));
    }

    diesel::update(crate::tables::crawler_queue::table)
    .set((
        crate::tables::crawler_queue::status.eq("crawling"),
        crate::tables::crawler_queue::crawling_node.eq("node-1"), // TODO: Get actual node ID.
        crate::tables::crawler_queue::crawling_since.eq(get_timestamp() as i64),
    ))
    .execute(&mut *db).expect("Failed to insert URL.");

    return status::Custom(Status::Ok, json!({
        "ok": true,
        "data": vec![queue_result.unwrap()]
    }));
}