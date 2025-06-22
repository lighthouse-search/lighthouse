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

use crate::global::{ generate_random_id, is_null_or_whitespace, request_authentication };
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

    for data in actions.clone() {
        let url: String = data.url.clone().expect("missing body.url");
        let content: Crawler_index_body_action_content = data.content.clone().expect("missing body.content");

        let created: i64 = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_millis().try_into().unwrap();

        let parsed_url = Url::parse(&url).expect("Failed to parse URL");
    
        ops.push(
            BulkOperation::create(json!({
                "url": url,
                "host": parsed_url.host_str(),
                "content": content,
                "indexed": created
            })
        )
        .id(url)
        // .pipeline("process_tweet")
        .into());
    };

    // Add bulk event data to elasticsearch.
    ES
    .bulk(BulkParts::Index("lighthouse-index"))
    .body(ops)
    .send()
    .await.expect("Failed to insert data.");

    return status::Custom(Status::Ok, json!({
        "ok": true
    }));
}