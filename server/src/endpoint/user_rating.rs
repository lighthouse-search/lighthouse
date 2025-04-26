use rocket::response::{status, status::Custom};
use rocket::http::Status;

use serde::{Serialize, Deserialize};
use serde_json::{Value, json};
use rocket::serde::json::Json;

use diesel::prelude::*;
use diesel::sql_types::*;
use diesel::sql_query;

use crate::global::{is_null_or_whitespace, request_authentication};
use crate::responses::*;
use crate::structs::*;
use crate::tables::*;

use uuid::Uuid;

#[get("/list?<id>&<filter>")]
pub async fn user_rating_list(params: &Query_string, id: Option<String>, filter: Option<String>) -> Custom<Value> {
    // TODO: THIS ISNT FILTERING BY ID AND ORG.
    // Get internal database information.
    let mut db = crate::DB_POOL.get().expect("Failed to get a connection from the pool.");
    
    if (id.is_none() == true) {
        return status::Custom(Status::BadRequest, not_found("params.ids is null or empty."));
    }

    let ids: Vec<String> = match id {
        Some(id_string) => {
            if (is_null_or_whitespace(Some(id_string.clone())) == true) {
                Vec::new()
            } else {
                // Split by commas if it's a list, or use as a single item
                id_string.split(',').map(|s| s.trim().to_string()).collect()
            }
        }
        None => Vec::new(), // No `id` provided
    };

    // Authenticated user.
    let request_authentication_output: Request_authentication_output = match request_authentication(None, params, "/user-rating/list").await {
        Ok(data) => data,
        Err(e) => return status::Custom(Status::Unauthorized, not_authorized())
    };

    let (user_rating_db, user_rating_counts_map, user_rating_me) = crate::globals::user_rating::get(db, ids, request_authentication_output.project_id.clone(), Some(request_authentication_output.account_id.clone())).await;
    db = user_rating_db;

    status::Custom(Status::Ok, json!({
        "ok": true,
        "data": user_rating_counts_map,
        "my_user_rating": user_rating_me
    }))
}

#[post("/update", format = "application/json", data = "<body>")]
pub async fn user_rating_update(params: &Query_string, mut body: Json<User_rating_update_body>) -> Custom<Value> {
    // TODO: This is half for events, half for discussion, fix it.
    let mut db = crate::DB_POOL.get().expect("Failed to get a connection from the pool.");

    let request_authentication_output: Request_authentication_output = match request_authentication(None, params, "/user-rating/update").await {
        Ok(data) => data,
        Err(e) => return status::Custom(Status::Unauthorized, not_authorized())
    };

    let actions = body.actions.clone().unwrap();

    for data in actions.clone() {
        // Normallly it would matter what the value of unwrap_or was here, since we're trying to check the original value, in this case checking if it's None, but it doesn't matter here because there is a check for 'create' or 'update'.
        let action = data.action.clone().unwrap_or(String::new());
        if (action != "create" && action != "remove") {
            return status::Custom(Status::BadRequest, error_message("invalid_value", "body.action must be create/remove."));
        }

        // TODO: Check this discussion ID already exists. We don't want arbitrary discussions being created.
        if (is_null_or_whitespace(data.id.clone()) == true) {
            // TODO: move this into an array and return errors after the validation look finishes, in bulk.
            return status::Custom(Status::BadRequest, error_message("is_null_or_whitespace", "body.id is null or whitespace."));
        }

        if (action == "create") {
            if (is_null_or_whitespace(data.emoji.clone()) == true) {
                // TODO: move this into an array and return errors after the validation look finishes, in bulk.
                return status::Custom(Status::BadRequest, error_message("is_null_or_whitespace", "body.emoji is null or whitespace."));
            }
        } else if (action == "remove") {

        }
    };

    for data in actions.clone() {
        let action = data.action.expect("missing body.action");

        if (action == "create") {
            let id = data.id.clone().expect("missing body.id");
            let emoji: String = data.emoji.clone().expect("missing body.emoji");

            // Delete any existing reaction from the account so they don't double-up.
            diesel::delete(user_rating::table.filter(user_rating::id.eq(id.clone()).and(user_rating::emoji.eq(emoji.clone())).and(user_rating::project.eq(request_authentication_output.project_id.clone())).and(user_rating::author.eq(request_authentication_output.account_id.clone()))))
                .execute(&mut db)
                .expect("Failed to delete discussion");

            let user_rating_insert = User_rating {
                // TODO: Ensure user can actually talk in this discussion. The discussion may be part of another project the user does not have access to. This could allow them to inject malicious content if it goes unpatched.
                id: id,
                emoji: emoji,
                author: request_authentication_output.account_id.clone(),
                project: request_authentication_output.project_id.clone(),
                created: Some(std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_millis().try_into().unwrap())
            };

            diesel::insert_into(user_rating::table)
            .values(&user_rating_insert)
            .execute(&mut db)
            .expect("fail");
        } else if (action == "remove") {
            let id = data.id.clone().expect("missing body.id");
            let emoji: String = data.emoji.clone().expect("missing body.emoji");
            
            diesel::delete(user_rating::table.filter(user_rating::id.eq(id)
                .and(user_rating::emoji.eq(emoji))
                .and(user_rating::project.eq(request_authentication_output.project_id.clone()))
                .and(user_rating::author.eq(request_authentication_output.account_id.clone()))))
                .execute(&mut db)
                .expect("Failed to delete discussion");
        }
    };

    return status::Custom(Status::Ok, json!({
        "ok": true
    }));
}