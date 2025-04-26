use rocket::response::{Debug, status::Created};
use rocket::response::status;
use rocket::http::Status;
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
use std::error::Error;
use std::process::{Command, Stdio};
use std::time::{SystemTime, UNIX_EPOCH};
use std::env;

use std::fs::{File};
use std::io::Write;

use rand::prelude::*;

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

pub fn stage() -> AdHoc {
    AdHoc::on_ignite("Diesel SQLite Stage", |rocket| async {
        rocket
        .mount("/api", routes![options_handler])
        .mount("/api/native-v1/query", routes![crate::endpoint::query::query_list])
        .mount("/api/native-v1/crawler", routes![crate::endpoint::crawler::crawler_index])
        // .mount("/api/native-v1/issue", routes![crate::endpoint::issue::issue_list, crate::endpoint::issue::issue_update])
        // .mount("/api/native-v1/event", routes![])
        // .mount("/api/native-v1/request", routes![])
        // .mount("/api/native-v1/timing", routes![])
        // .mount("/api/native-v1/tests", routes![])
        // .mount("/api/native-v1/discussion", routes![crate::endpoint::discussion::discussion_list, crate::endpoint::discussion::discussion_update])
        .mount("/api/native-v1/account", routes![crate::endpoint::account::account_me, crate::endpoint::account::account_list])
        // .mount("/api/native-v1/org", routes![crate::endpoint::org::org_list])
        // .mount("/api/native-v1/namespace", routes![crate::endpoint::namespace::namespace_list])
        // .mount("/api/native-v1/project", routes![crate::endpoint::project::project_list])
        // .mount("/api/native-v1/user-rating", routes![crate::endpoint::user_rating::user_rating_list, crate::endpoint::user_rating::user_rating_update])
        .mount("/api/native-v1/admin/index/job", routes![crate::endpoint::admin::index::admin_index_list, crate::endpoint::admin::index::admin_index_update])
    })
}