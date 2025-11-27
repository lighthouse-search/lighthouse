use rocket::response::{status, status::Custom};
use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::{get, post};

use serde::{Serialize, Deserialize};
use serde_json::{Value, json};

use diesel::prelude::*;
use diesel::sql_types::*;
use diesel::sql_query;

use crate::global::{generate_random_id, is_null_or_whitespace};
use crate::responses::*;
use crate::structs::*;
use crate::tables::*;

use std::net::SocketAddr;

use chrono::prelude::*;

pub fn unix_microseconds() -> i64 {
    return Utc::now().timestamp_micros();
}