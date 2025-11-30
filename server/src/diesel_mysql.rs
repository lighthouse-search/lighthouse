pub struct Cors;

use rocket::{options, routes, Response};
use rocket::response::{Debug, status::Created};
use rocket::response::status;
use rocket::http::{Header, Status};
use rocket::response::status::Custom;
use rocket::request::{self, Request, FromRequest};
use rocket::{fairing::{Fairing, Info, Kind}, State};
use rocket::fairing::AdHoc;
use rocket::fs::FileServer;

use serde::{Serialize, Deserialize};
use serde_json::{Value, json};
use rocket::serde::json::Json;

use diesel::prelude::*;
use diesel::sql_types::*;

use std::borrow::{Borrow, BorrowMut};
use std::collections::HashMap;
use std::error::Error;
use std::process::{Command, Stdio};
use std::time::{SystemTime, UNIX_EPOCH};
use std::env;

use std::fs::{File};
use std::io::Write;

use rand::prelude::*;

// use crate::endpoint::metadata::metadata_urls;
use crate::global::{ generate_random_id, is_null_or_whitespace, request_authentication };
use crate::responses::*;
use crate::structs::*;
use crate::tables::*;

use hades_auth::*;

use core::sync::atomic::{AtomicUsize, Ordering};

use diesel::mysql::MysqlConnection;
use diesel::r2d2::{self, ConnectionManager};
use std::sync::Arc;

#[options("/<_..>")]
fn options_handler() -> &'static str {
    ""
}

pub fn stage() -> AdHoc {
    AdHoc::on_ignite("Diesel SQLite Stage", |rocket| async {
        rocket
        .mount("/api", routes![options_handler])
        .mount("/", routes![crate::endpoint::misc::opensearch])
        // .mount("/api/native-v1/metadata", routes![metadata_urls])
        .mount("/api/native-v1/query", routes![crate::endpoint::query::query_list])
        .mount("/api/native-v1/crawler", routes![crate::endpoint::crawler::crawler_index, crate::endpoint::crawler::crawler_queue])
        .mount("/api/native-v1/account", routes![crate::endpoint::account::account_me, crate::endpoint::account::account_list])
        .mount("/api/native-v1/admin/index/job", routes![crate::endpoint::admin::index::admin_index_list, crate::endpoint::admin::index::admin_index_update])
    })
}

// Returns the current request's ID, assigning one only as necessary.
#[rocket::async_trait]
impl<'r> FromRequest<'r> for &'r Query_string {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        // The closure passed to `local_cache` will be executed at most once per
        // request: the first time the `RequestId` guard is used. If it is
        // requested again, `local_cache` will return the same value.

        request::Outcome::Success(request.local_cache(|| {
            let query_params = request.uri().query().map(|query| query.as_str().to_owned()).unwrap_or_else(|| String::new());

            Query_string(query_params)
        }))
    }
}

#[rocket::async_trait]
impl Fairing for Cors {
    fn info(&self) -> Info {
        Info {
            name: "Cross-Origin-Resource-Sharing Fairing",
            kind: Kind::Response,
        }
    }

    async fn on_response<'r>(&self, _request: &'r Request<'_>, response: &mut Response<'r>) {
        response.set_header(Header::new("Access-Control-Allow-Origin", "*"));
        response.set_header(Header::new(
            "Access-Control-Allow-Methods",
            "POST, PATCH, PUT, DELETE, HEAD, OPTIONS, GET",
        ));
        response.set_header(Header::new("Access-Control-Allow-Headers", "*"));
        response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
        response.remove_header("server");
    }
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for &'r Headers {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        request::Outcome::Success(request.local_cache(|| {
            let value = request.headers().iter()
                .map(|header| (header.name.to_string(), header.value.to_string()))
                .collect::<HashMap<String, String>>();

            Headers { headers_map: value }
        }))
    }
}