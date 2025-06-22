use rocket::response::{status, status::Custom};
use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::{get, post};

use serde::{Serialize, Deserialize};
use serde_json::{Value, json};

use diesel::prelude::*;
use diesel::sql_types::*;
use diesel::sql_query;

use crate::global::{is_null_or_whitespace, request_authentication};
use crate::responses::*;
use crate::structs::*;
use crate::tables::*;

#[get("/me")]
pub async fn account_me(params: &Query_string) -> Custom<Value> {
    // TODO: THIS ISNT FILTERING BY ID AND ORG.
    // Get internal database information.
    let mut db = crate::DB_POOL.get().expect("Failed to get a connection from the pool.");

    // Authenticated user.
    let request_authentication_output: Request_authentication_output = match request_authentication(None, params, "/account/me").await {
        Ok(data) => data,
        Err(e) => return status::Custom(Status::Unauthorized, not_authorized())
    };

    let account_result: Accounts = accounts::table
    .filter(crate::tables::accounts::id.eq(request_authentication_output.account_id.clone()))
    .select(
        crate::tables::accounts::all_columns,
    )
    .first::<Accounts>(&mut *db)
    .expect("Something went wrong querying the DB.");

    let Accounts_me: Accounts_me = account_result.into();

    status::Custom(Status::Ok, json!({
        "ok": true,
        "data": Accounts_me,
    }))
}

#[get("/list")]
pub async fn account_list(params: &Query_string) -> Custom<Value> {
    // Get internal database information.
    let mut db = crate::DB_POOL.get().expect("Failed to get a connection from the pool.");

    // Authenticated user.
    let request_authentication_output: Request_authentication_output = match request_authentication(None, params, "/account/list").await {
        Ok(data) => data,
        Err(e) => return status::Custom(Status::Unauthorized, not_authorized())
    };

    let account_result: Vec<Accounts> = accounts::table
    .select(
        crate::tables::accounts::all_columns,
    )
    .load::<Accounts>(&mut *db)
    .expect("Something went wrong querying the DB.");

    let mut account_public_result: Vec<Accounts_admin> = account_result
    .into_iter()
    .map(|(account)| {
        Accounts_admin::from(account)
    })
    .collect();

    status::Custom(Status::Ok, json!({
        "ok": true,
        "data": account_public_result,
    }))
}