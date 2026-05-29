use axum::extract::Query;
use axum::http::StatusCode;
use axum::Json;

use serde::Deserialize;
use serde_json::{Value, json};

use opensearch::SearchParts;

use crate::global::{ get_timestamp, is_null_or_whitespace };
use crate::responses::*;
use crate::structs::*;
use crate::ES;

#[derive(Debug, Default, Deserialize)]
pub struct Query_list_params {
    pub query: Option<String>,
    pub authenticator_pathname: Option<String>,
    pub filter: Option<String>,
}

pub async fn query_list(Query(params): Query<Query_list_params>) -> (StatusCode, Json<Value>) {
    let Query_list_params { query, authenticator_pathname: _, filter: _ } = params;
    let mut timing_markers = Vec::new();
    timing_markers.push(get_timestamp());

    // let request_authentication_output: Request_authentication_output = match request_authentication(None, &params, "/query/list").await {
    //     Ok(data) => data,
    //     Err(e) => return (StatusCode::UNAUTHORIZED, Json(not_authorized()))
    // };

    if is_null_or_whitespace(query.clone()) {
        return (StatusCode::BAD_REQUEST, Json(not_found("params.query is null or whitespace.")));
    }
    
    let query_unwrapped = query.unwrap();

    let should: Vec<Value> = vec![
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

    let query: Value = json!({
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
    .await.expect("Failed to query search backend");

    timing_markers.push(get_timestamp() - timing_markers[0]);

    let response_body = response.json::<Value>().await.expect("Failed to parse response.");
    // println!("response_body {}", response_body.clone());

    if response_body["error"].is_null() == false {
        println!("search backend returned an error: {}", response_body.clone());
        return (StatusCode::INTERNAL_SERVER_ERROR, Json(error_message("internal_server_error", "Sorry, something went wrong.")));
    }

    timing_markers.push(get_timestamp() - timing_markers[0]);

    let mut results: Vec<SearchResult> = Vec::new();
    if let Some(hits) = response_body["hits"]["hits"].as_array() {
        // println!("HITS: {:?}", hits.clone());
        for hit in hits {
            let response = hit["_source"].clone();
            let mut output: SearchResult = SearchResult {
                url: response["url"].as_str().unwrap().to_string(),
                title: None,
                text: None,
                favicon: None
            };
            
            if response["content"]["metatag"]["og:title"].is_null() == false  {
                output.title = response["content"]["metatag"]["og:title"].as_str().map(|s| s.to_string());
            } 
            
            if response["content"]["metatag"]["og:description"].is_null() == false  {
                output.text = response["content"]["metatag"]["og:description"].as_str().map(|s| s.to_string());
            } else {
                output.text = hit["highlight"]["content.text"][0].as_str().map(|s| s.to_string());
            }

            output.title = crate::globals::text::crop_string(output.title, 100);
            output.text = crate::globals::text::crop_string(output.text, 100);

            if response["content"]["linktag"]["icon"].is_null() == false {
                if response["content"]["linktag"]["icon"].clone().to_string().starts_with("http") == false && response["content"]["linktag"]["icon"].clone().to_string().starts_with("https://") == false {
                    let favicon_url = format!("https://{}{}", response["host"].as_str().unwrap(), response["content"]["linktag"]["icon"][0].clone().as_str().unwrap());
                    output.favicon = Some(favicon_url)
                } else {
                    output.favicon = response["content"]["linktag"]["icon"].as_str().map(|s| s.to_string());
                }
            }

            results.push(output);
        }
    }

    timing_markers.push(get_timestamp() - timing_markers[0]);
    timing_markers.push(get_timestamp());

    println!("timing_markers: {:?}", timing_markers.clone());

    (StatusCode::OK, Json(json!({
        "ok": true,
        "data": results,
        "stats": json!({
            "total": response_body["hits"]["total"]["value"],
            "took": response_body["took"]
        }),
        // "timing": timing_markers[timing_markers.len()-1] - timing_markers[0]
    })))
}