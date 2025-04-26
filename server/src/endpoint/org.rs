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

#[get("/list?<id>")]
pub async fn org_list(params: &Query_string, id: Option<String>) -> Custom<Value> {
    // TODO: THIS ISNT FILTERING BY ID AND ORG.
    // Get internal database information.
    let mut db = crate::DB_POOL.get().expect("Failed to get a connection from the pool.");

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
    if (ids.len() > 100) {
        return status::Custom(Status::BadRequest, not_found("params.ids cannot be longer than 100."));
    }

    // Authenticated user.
    let request_authentication_output: Request_authentication_output = match request_authentication(None, params, "/org/list").await {
        Ok(data) => data,
        Err(e) => return status::Custom(Status::Unauthorized, not_authorized())
    };

    let mut org_query = orgs::table
    .order(orgs::created.asc())
    .select(crate::tables::orgs::all_columns)
    .into_boxed();

    // TODO: This isn't filtering by orgs the user has access to.
    if (ids.len() > 0) {
        org_query = org_query.filter(crate::tables::orgs::id.eq_any(ids.clone()))
    }

    let org_result = org_query
    // .limit(100)
    .order(orgs::created.asc())
    .load::<Org>(&mut *db)
    .expect("Something went wrong querying the DB.");

    let mut org_public_result: Vec<Org_public> = org_result
    .into_iter()
    .map(|org| {
        Org_public::from(org)
    })
    .collect();

    status::Custom(Status::Ok, json!({
        "ok": true,
        "data": org_public_result
    }))
}