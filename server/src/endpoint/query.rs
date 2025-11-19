use rocket::form::validate::Contains;
use rocket::response::{Debug, status::Created, status};
use rocket::http::Status;
use rocket::response::status::Custom;
use rocket::serde::json::Json;
use rocket::{get, post};

use serde::{Serialize, Deserialize};
use serde_json::{Value, json};

use diesel::prelude::*;
use diesel::sql_types::*;
use diesel::sql_query;

use elasticsearch::{BulkOperation, BulkParts, SearchParts};

use crate::database::elasticsearch_parse_response;
use crate::global::{ generate_random_id, is_null_or_whitespace, request_authentication };
use crate::responses::*;
use crate::structs::*;
use crate::tables::*;
use crate::ES;

use uuid::Uuid;

#[get("/list?<query>&<authenticator_pathname>&<filter>")]
pub async fn query_list(query: Option<String>, authenticator_pathname: Option<String>, filter: Option<String>, params: &Query_string) -> Custom<Value> {
    let mut db = crate::DB_POOL.get().expect("Failed to get a connection from the pool.");
    // TODO: This should have a dedicated function like video_get.

    // let request_authentication_output: Request_authentication_output = match request_authentication(None, params, "/query/list").await {
    //     Ok(data) => data,
    //     Err(e) => return status::Custom(Status::Unauthorized, not_authorized())
    // };

    if (is_null_or_whitespace(query.clone())) {
        return status::Custom(Status::BadRequest, not_found("params.query is null or whitespace."));
    }
    
    let query_unwrapped = query.unwrap();

    let mut should: Vec<Value> = vec![
        json!({
            "bool": {
                "should": [
                    { "wildcard": { "content.title": format!("*{}*", query_unwrapped.clone()) } },
                    { "match_phrase": { "content.text": query_unwrapped.clone() } },
                    { "match_phrase": { "url": query_unwrapped.clone() } }
                ],
                "minimum_should_match": 1
            }
        })
    ];

    // json!({
    //     "term": json!({
    //         "url": query_unwrapped.clone(),
    //     })
    // })

    // let mut discussion_output: Option<std::collections::HashMap<String, i64>> = None;
    let _source: Vec<&str> = vec!["url", "content.title", "content.text", "content.urls", "content.metatag", "indexed"];

    let mut query: Value = json!({
        "track_total_hits": true,
        "size": 100,
        "sort": [
            { "indexed": { "order": "asc" } }
        ],
        "query": {
            "bool": {
                "should": should
            }
        },
        "highlight": {
            "fields": {
                "content.text": {}
            }
        },
        "_source": _source
    });
    // if (aggs.is_none() == false) {
    //     query["aggs"] = aggs.clone().unwrap();
    // }

    let response = ES
    .search(SearchParts::Index(&["lighthouse-index"]))
    .from(0)
    .body(query)
    .send()
    .await.expect("Failed to query ElasticSearch");

    let response_body = response.json::<Value>().await.expect("Failed to parse response.");
    println!("response_body {}", response_body.clone());

    if (response_body["error"].is_null() == false) {
        println!("elasticsearch returned an error: {}", response_body.clone());
        return status::Custom(Status::InternalServerError, error_message("internal_server_error", "Sorry, something went wrong."));
    }

    let mut results: Vec<Value> = Vec::new();
    if let Some(hits) = response_body["hits"]["hits"].as_array() {
        // println!("HITS: {:?}", hits.clone());
        for hit in hits {
            let mut result = hit["_source"].clone();
            if (result["content"]["metatag"]["og:title"].is_null() == false) {
                result["content"]["title"] = result["content"]["metatag"]["og:title"].clone();
            } 
            
            if (result["content"]["metatag"]["og:description"].is_null() == false) {
                result["content"]["text"] = result["content"]["metatag"]["og:description"].clone();
            } else {
                result["content"]["text"] = hit["highlight"]["content.text"].clone();
            }
            results.push(json!(result));
        }
    }

    status::Custom(Status::Ok, json!({
        "ok": true,
        "data": results,
        "took": response_body["took"]
    }))
}