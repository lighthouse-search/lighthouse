use axum::http::StatusCode;
use axum::Json;

use serde_json::{Value, json};

use diesel::prelude::*;

use crate::global::request_authentication;
use crate::responses::*;
use crate::structs::*;
use crate::tables::*;

pub async fn account_me(params: Query_string) -> (StatusCode, Json<Value>) {
    // TODO: THIS ISNT FILTERING BY ID AND ORG.
    // Get internal database information.
    let mut db = crate::DB_POOL.get().expect("Failed to get a connection from the pool.");

    // Authenticated user.
    let request_authentication_output: Request_authentication_output = match request_authentication(None, &params, "/account/me").await {
        Ok(data) => data,
        Err(_e) => return (StatusCode::UNAUTHORIZED, Json(not_authorized()))
    };

    let account_result: Accounts = accounts::table
    .filter(crate::tables::accounts::id.eq(request_authentication_output.account_id.clone()))
    .select(
        crate::tables::accounts::all_columns,
    )
    .first::<Accounts>(&mut *db)
    .expect("Something went wrong querying the DB.");

    let accounts_me: Accounts_me = account_result.into();

    (StatusCode::OK, Json(json!({
        "ok": true,
        "data": accounts_me,
    })))
}

pub async fn account_list(params: Query_string) -> (StatusCode, Json<Value>) {
    // Get internal database information.
    let mut db = crate::DB_POOL.get().expect("Failed to get a connection from the pool.");

    // Authenticated user.
    let _request_authentication_output: Request_authentication_output = match request_authentication(None, &params, "/account/list").await {
        Ok(data) => data,
        Err(_e) => return (StatusCode::UNAUTHORIZED, Json(not_authorized()))
    };

    let account_result: Vec<Accounts> = accounts::table
    .select(
        crate::tables::accounts::all_columns,
    )
    .load::<Accounts>(&mut *db)
    .expect("Something went wrong querying the DB.");

    let account_public_result: Vec<Accounts_admin> = account_result
    .into_iter()
    .map(Accounts_admin::from)
    .collect();

    (StatusCode::OK, Json(json!({
        "ok": true,
        "data": account_public_result,
    })))
}