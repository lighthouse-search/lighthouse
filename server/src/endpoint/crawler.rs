use axum::http::StatusCode;
use axum::Json;

use serde_json::{Value, json};

use diesel::prelude::*;

use crate::global::{ get_timestamp, is_null_or_whitespace };
use crate::structs::*;
use crate::ES;

use opensearch::{BulkOperation, BulkParts};
use url::Url;

pub async fn crawler_index(Json(body): Json<Crawler_index_body>) -> (StatusCode, Json<Value>) {
    let mut db = crate::DB_POOL.get().expect("Failed to get a connection from the pool.");

    // // TODO: This authentication needs to be locked to crawlers only.
    // let request_authentication_output: Request_authentication_output = match request_authentication(None, params, "/crawler/index").await {
    //     Ok(data) => data,
    //     Err(e) => return status::Custom(Status::Unauthorized, not_authorized())
    // };

    let actions = body.actions.clone().unwrap();

    // TODO: Return errors here.
    for data in actions.clone() {
        if is_null_or_whitespace(data.url.clone())  {
            
        }

        if data.content.is_none() == true  {
            
        }
        let content_unwrapped: Crawler_index_body_action_content = data.content.clone().expect("missing body.content");
        if is_null_or_whitespace(content_unwrapped.title.clone()) == true  {

        }
        if is_null_or_whitespace(content_unwrapped.text.clone()) == true  {

        }
        if content_unwrapped.urls.is_none() == true || content_unwrapped.urls.clone().unwrap().len() == 0  {

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
            if url_queue.iter().position(|x| &x.url == link).is_some()  {
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
            BulkOperation::create(
                url.clone(),
                json!({
                    "url": url.clone(),
                    "host": parsed_url.host_str(),
                    "content": content,
                    "indexed": created
                })
            ).into()
        );
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

    return (StatusCode::OK, Json(json!({
        "ok": true
    })));
}

pub async fn crawler_queue() -> (StatusCode, Json<Value>) {
    let mut db = crate::DB_POOL.get().expect("Failed to get a connection from the pool.");

    let node_id = std::env::var("lighthouse_node_id").unwrap_or_else(|_| "node-1".to_string());

    // Atomically claim the next queued URL for this node (see crawl::queue).
    match crate::crawl::queue::claim_next_url(&mut *db, &node_id) {
        Some(item) => (StatusCode::OK, Json(json!({
            "ok": true,
            "data": vec![item]
        }))),
        None => (StatusCode::OK, Json(json!({
            "ok": true,
            "data": Vec::<Crawler_queue>::new()
        }))),
    }
}