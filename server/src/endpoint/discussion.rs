use serde::{Serialize, Deserialize};
use serde_json::{Value, json};
use rocket::serde::json::Json;

use rocket::response::{status, status::Custom};
use rocket::http::Status;

use diesel::prelude::*;
use diesel::sql_types::*;
use diesel::sql_query;

use crate::global::{is_null_or_whitespace, request_authentication};
use crate::responses::*;
use crate::structs::*;
use crate::tables::*;

use uuid::Uuid;

#[get("/list?<id>&<filter>")]
pub async fn discussion_list(params: &Query_string, id: Option<String>, filter: Option<String>) -> Custom<Value> {
    // TODO: THIS ISNT FILTERING BY ID AND ORG.
    // Get internal database information.
    let mut db = crate::DB_POOL.get().expect("Failed to get a connection from the pool.");
    
    let filter_unwrapped: Discussion_list_filter = serde_json::from_str(&filter.unwrap()).unwrap_or(Discussion_list_filter {
        nonce: None
    });

    if (is_null_or_whitespace(id.clone()) == true && filter_unwrapped.nonce.is_none() == true) {
        return status::Custom(Status::BadRequest, not_found("params.ids and params.filter.nonces is null or empty."));
    }

    // Authenticated user.
    let request_authentication_output: Request_authentication_output = match request_authentication(None, params, "/discussion/list").await {
        Ok(data) => data,
        Err(e) => return status::Custom(Status::Unauthorized, not_authorized())
    };

    let (db_discussion, discussions_public, discussion_counts_map, discussions_distinct_authors_public) = crate::globals::discussion::get(db, id, filter_unwrapped.nonce, request_authentication_output.project_id).await;
    db = db_discussion;

    status::Custom(Status::Ok, json!({
        "ok": true,
        "data": discussions_public,
        "total": discussion_counts_map,
        "authors": discussions_distinct_authors_public
    }))
}

#[post("/update", format = "application/json", data = "<body>")]
pub async fn discussion_update(params: &Query_string, mut body: Json<Discussion_update_body>) -> Custom<Value> {
    // TODO: This is half for events, half for discussion, fix it.
    let mut db = crate::DB_POOL.get().expect("Failed to get a connection from the pool.");

    let request_authentication_output: Request_authentication_output = match request_authentication(None, params, "/discussion/update").await {
        Ok(data) => data,
        Err(e) => return status::Custom(Status::Unauthorized, not_authorized())
    };

    let actions = body.actions.clone().unwrap();

    for data in actions.clone() {
        // Normallly it would matter what the value of unwrap_or was here, since we're trying to check the original value, in this case checking if it's None, but it doesn't matter here because there is a check for 'create' or 'update'.
        let action = data.action.clone().unwrap_or(String::new());
        if (action != "create") { // && action != "update"
            return status::Custom(Status::BadRequest, error_message("invalid_value", "body.action must be create/update."));
        }

        // TODO: Check this discussion ID already exists. We don't want arbitrary discussions being created.
        if (is_null_or_whitespace(data.id.clone()) == true) {
            // TODO: move this into an array and return errors after the validation look finishes, in bulk.
            return status::Custom(Status::BadRequest, error_message("is_null_or_whitespace", "body.id is null or whitespace."));
        }

        // TODO: Accept error data as an event as add it to this error. Most likely best to do this when handling incoming events.
        if (action == "create") {
            if (is_null_or_whitespace(data.content.clone()) == true) {
                // TODO: move this into an array and return errors after the validation look finishes, in bulk.
                return status::Custom(Status::BadRequest, error_message("is_null_or_whitespace", "body.content is null or whitespace."));
            }
        } else if (action == "update") {
            // TODO: This needs to check discussions, not devices, and add an edit history.
            let error_id = data.id.clone().expect("missing body.id");
            let device_check: Option<Issue> = issue::table
            .filter(issue::id.eq(error_id.clone()))
            .first(&mut db)
            .optional()
            .expect("Something went wrong querying the DB.");
        }
    };

    for data in actions.clone() {
        let discussion_id = data.id.clone().expect("missing body.id");
        let project_id = request_authentication_output.project_id.clone();
        let content: Option<String> = Some(data.content.clone().expect("missing body.content"));

        // TODO: need to do permission checks on things like events-[hash] and other nonces throughout the app. Someone without permission could inject messages into discussions.
        let nonce: Option<String> = data.nonce;

        // TODO: Validate these attachments.
        let attachments = data.attachments;

        let discussion_insert = Discussions {
            // TODO: Ensure user can actually talk in this discussion. The discussion may be part of another project the user does not have access to. This could allow them to inject malicious content if it goes unpatched.
            message_id: Uuid::new_v4().to_string(),
            discussion: discussion_id,
            author: request_authentication_output.account_id.clone(),
            project: project_id,
            content: content,
            attachments: attachments,
            nonce: nonce,
            created: Some(std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_millis().try_into().unwrap())
        };

        diesel::insert_into(discussions::table)
        .values(&discussion_insert)
        .execute(&mut db)
        .expect("fail");
    };

    return status::Custom(Status::Ok, json!({
        "ok": true
    }));
}