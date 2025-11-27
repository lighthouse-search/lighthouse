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

// Runners ping this endpoint to update job status
#[post("/update", format = "application/json", data = "<body>")]
pub async fn job_update(mut body: Json<Job_update_body>, params: &Query_string, remote_addr: SocketAddr) -> Custom<Value> {
    let mut db = crate::DB_POOL.get().expect("Failed to get a connection from the pool.");

    // TODO: Get current job data here, e.g. through a crate::misc::job function.
    // If a piece of data isn't being updated, we need to substitute the existing value into the UPDATE query. It's cleaner than trying to conditionally update values.
    diesel::update(
        crate::tables::jobs::table.filter(
            crate::tables::jobs::id.eq(body.job_id.clone().unwrap())
            .and(crate::tables::jobs::runner_ip.eq(remote_addr.ip().to_string())
            .and(crate::tables::jobs::nonce.eq(body.nonce.clone().unwrap())))
        )
    )
    .set((jobs::status.eq(body.status.clone().unwrap())))
    .execute(&mut db).expect("Failed to update");

    status::Custom(Status::Ok, json!({
        "ok": true
    }))
}
