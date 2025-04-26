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
pub async fn namespace_list(params: &Query_string, id: Option<String>) -> Custom<Value> {
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
    let request_authentication_output: Request_authentication_output = match request_authentication(None, params, "/namespace/list").await {
        Ok(data) => data,
        Err(e) => return status::Custom(Status::Unauthorized, not_authorized())
    };

    let mut org_query = namespaces::table
    .left_join(crate::tables::orgs::dsl::orgs.on(crate::tables::orgs::dsl::id.nullable().eq(crate::tables::namespaces::dsl::org.nullable())))
    .order(namespaces::created.asc())
    .select((
        crate::tables::namespaces::all_columns,
        crate::tables::orgs::all_columns.nullable(),
    ))
    .into_boxed();

    // TODO: This isn't filtering by orgs the user has access to.
    if (ids.len() > 0) {
        org_query = org_query.filter(crate::tables::namespaces::id.eq_any(ids.clone()))
    }

    let namespace_result = org_query
    // .limit(100)
    .order(namespaces::created.asc())
    .load::<(Namespace, Option<Org>)>(&mut *db)
    .expect("Something went wrong querying the DB.");

    let mut namespace_public_result: Vec<Namespace_public> = namespace_result
    .into_iter()
    .map(|(namespace, org)| {
        Namespace_public::from((namespace, org.unwrap()))
    })
    .collect();

    status::Custom(Status::Ok, json!({
        "ok": true,
        "data": namespace_public_result
    }))
}