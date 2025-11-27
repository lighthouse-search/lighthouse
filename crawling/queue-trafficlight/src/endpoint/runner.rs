use rocket::response::{status, status::Custom};
use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::{get, post};

use serde::{Serialize, Deserialize};
use serde_json::{Value, json};

use diesel::prelude::*;
use diesel::sql_types::*;
use diesel::sql_query;

use crate::global::is_null_or_whitespace;
use crate::responses::*;
use crate::structs::*;
use crate::tables::*;

use std::net::SocketAddr;

// Runners ping this endpoint at boot to signal to the traffic light service they're ready to receive jobs.
#[post("/hello", format = "application/json", data = "<body>")]
pub async fn runner_hello(mut body: Json<Runner_hello_body>, params: &Query_string, remote_addr: SocketAddr) -> Custom<Value> {
    let mut db = crate::DB_POOL.get().expect("Failed to get a connection from the pool.");

    // SQL expects BIGINT, which translates to i64 in Rust. Body.capacity is u64. If body.capacity is specified, convert it to i64.
    let capacity: Option<i64> = body.capacity.and_then(|c| Some(i64::try_from(c).unwrap()));

    let runner_insert = Runner {
        runner_ip: remote_addr.ip().to_string(),
        nonce: body.nonce.clone().unwrap(),
        capacity: capacity
    };
    diesel::insert_into(runner::table)
    .values(&runner_insert)
    .execute(&mut db)
    .expect("fail");

    status::Custom(Status::Ok, json!({
        "ok": true,
    }))
}

// Runners ping this endpoint when their capacity changes significantly.
#[post("/capacity/update", format = "application/json", data = "<body>")]
pub async fn runner_capacity_update(mut body: Json<Runner_capacity_body>, params: &Query_string, remote_addr: SocketAddr) -> Custom<Value> {
    let mut db = crate::DB_POOL.get().expect("Failed to get a connection from the pool.");

    // SQL expects BIGINT, which translates to i64 in Rust. Body.capacity is u64. If body.capacity is specified, convert it to i64.
    let capacity: Option<i64> = body.capacity.and_then(|c| Some(i64::try_from(c).unwrap()));

    diesel::update(crate::tables::runner::table.filter(crate::tables::runner::runner_ip.eq(remote_addr.ip().to_string()).and(crate::tables::runner::nonce.eq(body.nonce.clone().unwrap()))))
    .set((runner::capacity.eq(capacity)))
    .execute(&mut db).expect("Failed to update");

    status::Custom(Status::Ok, json!({
        "ok": true
    }))
}