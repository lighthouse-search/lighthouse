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
use crate::guard::guard_hostname_for_host;
use crate::{responses::*, GUARD_HOSTNAME_TO_USE};
use crate::structs::*;
use crate::tables::*;

#[get("/urls")]
pub async fn metadata_urls(params: &Query_string, headers: &Headers) -> Custom<Value> {
    
    status::Custom(Status::Ok, json!({
        "ok": true,
        "data": json!({
            "guard": guard_hostname_for_host(headers).expect("Failed to get guard hostname for host")
        })
    }))
}