use axum::extract::Query;
use axum::http::StatusCode;
use axum::Json;

use serde::Deserialize;
use serde_json::{Value, json};

use crate::global::{ is_null_or_whitespace, request_authentication };
use crate::responses::*;
use crate::structs::*;
use crate::ES;

use opensearch::{BulkOperation, BulkParts, SearchParts, DeleteByQueryParts};

use uuid::Uuid;
use url::Url;

#[derive(Debug, Default, Deserialize)]
pub struct Admin_index_list_params {
    pub ids: Option<String>,
    pub me: Option<bool>,
    pub authenticator_pathname: Option<String>,
}

pub async fn admin_index_list(params: Query_string, Query(query_params): Query<Admin_index_list_params>) -> (StatusCode, Json<Value>) {
    let Admin_index_list_params { ids, me: _, authenticator_pathname: _ } = query_params;
    // TODO: This should have a dedicated function like video_get.

    // TODO: This isn't verifying administrator permissions.
    let request_authentication_output: Request_authentication_output = match request_authentication(None, &params, "/admin/index/job/list").await {
        Ok(data) => data,
        Err(_e) => return (StatusCode::UNAUTHORIZED, Json(not_authorized()))
    };
    let _account_id = Some(request_authentication_output.account_id.clone());

    let id: Vec<String> = match ids {
        Some(id_string) => {
            if is_null_or_whitespace(Some(id_string.clone())) == true  {
                Vec::new()
            } else {
                // Split by commas if it's a list, or use as a single item
                id_string.split(',').map(|s| s.trim().to_string()).collect()
            }
        }
        None => Vec::new(), // No `id` provided
    };

    if id.len() > 100 {
        return (StatusCode::BAD_REQUEST, Json(not_found("params.ids cannot be longer than 100.")));
    }
    println!("id.len(): {}", id.len());

    let _source: Vec<&str> = vec!["status", "urls", "created"];

    let query: Value = json!({
        // "query": {
        //     "bool": {
        //         "must": {
        //             "PostDoc.text": params.q.unwrap()
        //         }
        //     }
        // },
        "_source": _source
    });
    // if (aggs.is_none() == false) {
    //     query["aggs"] = aggs.clone().unwrap();
    // }

    let response = ES
    .search(SearchParts::Index(&["lighthouse-index-jobs"]))
    .from(0)
    .body(query)
    .send()
    .await.expect("Failed to query search backend");

    let response_body = response.json::<Value>().await.expect("Failed to parse response.");
    println!("response_body {}", response_body.clone());

    if response_body["error"].is_null() == false {
        println!("search backend returned an error: {}", response_body.clone());
        return (StatusCode::INTERNAL_SERVER_ERROR, Json(error_message("internal_server_error", "Sorry, something went wrong.")));
    }

    let mut results: Vec<Value> = Vec::new();
    if let Some(hits) = response_body["hits"]["hits"].as_array() {
        // println!("HITS: {:?}", hits.clone());
        for hit in hits {
            let mut result = hit["_source"].clone();
            result["id"] = hit["_id"].clone();
            results.push(json!(result));
        }
    }

    (StatusCode::OK, Json(json!({
        "ok": true,
        "data": results,
        "took": response_body["took"]
    })))
}

pub async fn admin_index_update(params: Query_string, Json(body): Json<Admin_index_update_body>) -> (StatusCode, Json<Value>) {
    let _request_authentication_output: Request_authentication_output = match request_authentication(None, &params, "/admin/index/job/update").await {
        Ok(data) => data,
        Err(_e) => return (StatusCode::UNAUTHORIZED, Json(not_authorized()))
    };

    let actions = body.actions.clone().unwrap();

    let mut actions_create: Vec<Admin_index_update_action> = Vec::new();
    let mut actions_delete: Vec<Admin_index_update_action> = Vec::new();

    // TODO: Errors here should return an array of errors, not break the validation string and return 1 by 1.
    let mut action_pos = -1;
    for data in actions.clone() {
        action_pos = action_pos+1;
        // Normallly it would matter what the value of unwrap_or was here, since we're trying to check the original value, in this case checking if it's None, but it doesn't matter here because there is a check for 'create' or 'update'.
        let action = data.action.clone().unwrap_or(String::new());

        if action == "create" {
            if data.url.is_none() == true || data.url.clone().unwrap().len() == 0 {
                return (StatusCode::BAD_REQUEST, Json(error_message("invalid_value", &format!("body.action[{}].url cannot be empty.", action_pos))));
            }

            let mut pos = -1;
            for url in data.url.clone().unwrap() {
                pos = pos+1;

                if url.starts_with("http://") == false && url.starts_with("https://") == false {
                    return (StatusCode::BAD_REQUEST, Json(error_message("invalid_value", &format!("body.action[{}].url[{}] must start with https or http.", action_pos, pos))));
                }
                let parsed_url = Url::parse(&url);
                if parsed_url.is_err() == true {
                    return (StatusCode::BAD_REQUEST, Json(error_message("invalid_value", &format!("body.action[{}].url[{}] is not a valid URL.", action_pos, pos))));
                }
                let parsed_url_unwrapped = Url::parse(&url).unwrap();
                if parsed_url_unwrapped.scheme() != "https" && parsed_url_unwrapped.scheme() != "http" {
                    return (StatusCode::BAD_REQUEST, Json(error_message("invalid_value", &format!("body.action[{}].url[{}] must use https or http.", action_pos, pos))));
                }
            }

            actions_create.push(data.clone());
        } else if action == "delete" {
            if data.id.is_none() {
                return (StatusCode::BAD_REQUEST, Json(error_message("invalid_value", &format!("body.action[{}].id cannot be empty.", action_pos))));
            }

            actions_delete.push(data.clone());
        } else {
            return (StatusCode::BAD_REQUEST, Json(error_message("invalid_value", &format!("body.action[{}].action must be create/delete.", action_pos))));
        }
    };

    // Delete jobs
    let mut delete_ids: Vec<String> = Vec::new();
    for data in actions_delete.clone() {
        let id: String = data.id.clone().expect("missing body.url");
        delete_ids.push(id);
    };

    // TODO: need to make decision of canceling should also remove items indexed under this job from the index. Could just make another API endpoint that deletes index data based on job_id instead.
    let query = json!({
        "query": {
            "bool": {
                "must": [
                    {
                        "terms": {
                            "_id": delete_ids
                        }
                    },
                        {
                        "terms": {
                            "status": ["pending"]
                        }
                    }
                ]
            }
        }
    });

    // TODO: need an elasticsearch pipeline here that checks for an error response.
    let response = ES
    .delete_by_query(DeleteByQueryParts::Index(&["lighthouse-index-jobs"]))
    .body(query)
    .send()
    .await.expect("Failed to delete search backend records");

    println!("response: {:?}", response.json::<Value>().await.expect("Failed to parse response."));

    // Create jobs
    let mut ops: Vec<BulkOperation<Value>> = Vec::with_capacity(actions.len());

    for data in actions_create.clone() {
        let url = data.url.clone().expect("missing body.url");

        let created: i64 = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_millis().try_into().unwrap();

        ops.push(
            BulkOperation::create(
                Uuid::new_v4().to_string(),
                json!({
                    "status": "pending",
                    "urls": url,
                    "created": created
                })
            ).into()
        );
    };

    // Add bulk event data to elasticsearch.
    // TODO: need an elasticsearch pipeline here that checks for an error response.
    ES
    .bulk(BulkParts::Index("lighthouse-index-jobs"))
    .body(ops)
    .send()
    .await.expect("Failed to insert data.");

    return (StatusCode::OK, Json(json!({
        "ok": true
    })));
}