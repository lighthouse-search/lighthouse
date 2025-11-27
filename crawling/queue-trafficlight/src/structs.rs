use std::collections::HashMap;

use diesel::prelude::*;
use crate::tables::*;
use diesel::r2d2::{self, ConnectionManager};

use serde::{Serialize, Deserialize};
use serde_json::{Value, json};
use rocket::serde::json::Json;

#[derive(Clone, Debug, Deserialize)]
pub struct Config {
    pub features: Option<Config_features>,
    pub runner: Option<Config_runners>,
    pub cursor: Option<Config_cursors>,
    pub database: Option<Config_database>
}

#[derive(Clone, Debug, Deserialize)]
pub struct Config_features {
    pub authentication: Option<bool>
}

#[derive(Clone, Debug, Deserialize)]
pub struct Config_runners {
    pub max_jobs: Option<u64>
}

#[derive(Clone, Debug, Deserialize)]
pub struct Config_cursors {
    pub max_assignable_cursor: Option<u64>,
    pub max_backfill: Option<u64>
}

#[derive(Clone, Debug, Deserialize)]
pub struct Config_database {
    pub mysql: Option<Config_database_mysql>
}

#[derive(Clone, Debug, Deserialize)]
pub struct Config_database_mysql {
    pub hostname: Option<String>,
    pub port: Option<u16>,
    pub username: Option<String>,
    pub password_env: Option<String>,
    pub database: Option<String>
}

// Internal structs
#[derive(Debug)]
pub struct Query_string(pub String);

#[derive(Debug, Clone, Deserialize, Serialize, Queryable, Insertable, Selectable, QueryableByName)]
#[serde(crate = "rocket::serde")]
#[diesel(table_name = crawler_runner)]
pub struct Runner {
    pub runner_ip: String,
    pub nonce: String,
    pub capacity: Option<i64>
}

#[derive(Debug, Clone, Deserialize, Serialize, Queryable, Insertable, Selectable, QueryableByName)]
#[serde(crate = "rocket::serde")]
#[diesel(table_name = crawler_jobs)]
pub struct Jobs {
    pub id: String,
    pub runner_ip: String,
    pub nonce: String,
    pub status: Option<String>,
    pub cursor_currently: Option<i64>,
    pub cursor_from: Option<i64>,
    pub cursor_to: Option<i64>
}

#[derive(Debug, Clone, Deserialize, Serialize, Queryable, Insertable, Selectable, QueryableByName)]
#[serde(crate = "rocket::serde")]
#[diesel(table_name = crawler_missingranges)]
pub struct Missing_ranges {
    pub missing_from: i64,
    pub missing_to: i64,
}

pub struct Headers {
    pub headers_map: HashMap<String, String>,
}

// Request Bodies
#[derive(Clone, Debug, Deserialize)]
pub struct Runner_hello_body {
    pub nonce: Option<String>,
    pub capacity: Option<u64>
}

#[derive(Clone, Debug, Deserialize)]
pub struct Runner_capacity_body {
    pub nonce: Option<String>,
    pub capacity: Option<u64>
}

#[derive(Clone, Debug, Deserialize)]
pub struct Runner_available {
    pub runner: Runner,
    pub job_count: i64,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Job_update_body {
    pub job_id: Option<String>,
    pub nonce: Option<String>,
    pub status: Option<String>
}