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
use crate::global::{ generate_random_id, get_timestamp, is_null_or_whitespace, request_authentication };
use crate::responses::*;
use crate::structs::*;
use crate::tables::*;
use crate::ES;

use uuid::Uuid;

#[get("/list?<query>&<authenticator_pathname>&<filter>")]
pub async fn query_list(query: Option<String>, authenticator_pathname: Option<String>, filter: Option<String>, params: &Query_string) -> Custom<Value> {
    let mut timing_markers = Vec::new();
    timing_markers.push(get_timestamp());
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
                    // { "wildcard": { "content.title": format!("*{}*", query_unwrapped.clone()) } },
                    { "match": { "content.title": query_unwrapped.clone() } },
                    { "match_phrase": { "content.text": query_unwrapped.clone() } },
                    { "match_phrase": { "url": query_unwrapped.clone() } }
                ],
                "minimum_should_match": 1
            }
        })
    ];

    timing_markers.push(get_timestamp() - timing_markers[0]);

    // json!({
    //     "term": json!({
    //         "url": query_unwrapped.clone(),
    //     })
    // })

    // let mut discussion_output: Option<std::collections::HashMap<String, i64>> = None;
    let _source: Vec<&str> = vec!["host", "url", "content.title", "content.text", "content.urls", "content.metatag", "content.linktag", "indexed"];

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
                "content.text": {},
            }
        },
        "_source": _source
    });
    // if (aggs.is_none() == false) {
    //     query["aggs"] = aggs.clone().unwrap();
    // }

    timing_markers.push(get_timestamp() - timing_markers[0]);

    let response = ES
    .search(SearchParts::Index(&["lighthouse-index"]))
    .from(0)
    .body(query)
    .send()
    .await.expect("Failed to query ElasticSearch");

    timing_markers.push(get_timestamp() - timing_markers[0]);

    let response_body = response.json::<Value>().await.expect("Failed to parse response.");
    println!("response_body {}", response_body.clone());

    if (response_body["error"].is_null() == false) {
        println!("elasticsearch returned an error: {}", response_body.clone());
        return status::Custom(Status::InternalServerError, error_message("internal_server_error", "Sorry, something went wrong."));
    }

    timing_markers.push(get_timestamp() - timing_markers[0]);

    let mut results: Vec<Value> = Vec::new();
    if let Some(hits) = response_body["hits"]["hits"].as_array() {
        // println!("HITS: {:?}", hits.clone());
        for hit in hits {
            let result = hit["_source"].clone();
            let mut output = json!({});
            if (result["content"]["metatag"]["og:title"].is_null() == false) {
                output["title"] = result["content"]["metatag"]["og:title"].clone();
            } 
            
            if (result["content"]["metatag"]["og:description"].is_null() == false) {
                output["text"] = result["content"]["metatag"]["og:description"].clone();
            } else {
                output["text"] = hit["highlight"]["content.text"].clone();
            }

            if (result["content"]["linktag"]["icon"].is_null() == false) {
                if (result["content"]["linktag"]["icon"].clone().to_string().starts_with("http") == false && result["content"]["linktag"]["icon"].clone().to_string().starts_with("https://") == false) {
                    output["favicon"] = format!("https://{}{}", result["host"].as_str().unwrap(), result["content"]["linktag"]["icon"][0].clone().as_str().unwrap()).into()
                } else {
                    output["favicon"] = result["content"]["linktag"]["icon"].clone();
                }
            }

            output["url"] = result["url"].clone();
            results.push(json!(output));
        }
    }

    timing_markers.push(get_timestamp() - timing_markers[0]);
    timing_markers.push(get_timestamp());

    println!("timing_markers: {:?}", timing_markers.clone());

    status::Custom(Status::Ok, json!({
        "ok": true,
        "data": results,
        "stats": json!({
            "total": response_body["hits"]["total"]["value"],
            "took": response_body["took"]
        }),
        // "timing": timing_markers[timing_markers.len()-1] - timing_markers[0]
    }))
}