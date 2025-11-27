mod diesel_mysql;
mod global;
mod structs;
mod responses;
mod tables;
mod database;
mod security;

pub mod globals {
    pub mod environment_variables;
}

pub mod endpoint {
    pub mod job;
    pub mod runner;
}

pub mod misc {
    pub mod cursor;
    pub mod job;
    pub mod missing_range;
    pub mod runner;
}

use diesel_mysql::Cors;
use rocket::fairing::{Fairing, Info, Kind};
use rocket::http::Header;
use rocket::{catch, catchers, Build, Rocket};
use rocket::{Request, Response, request, request::FromRequest};
// use websocket::connection::handle_connection;

use core::time;
use std::error::Error;
use std::fs;
use std::collections::HashMap;
use std::env;
use std::path::PathBuf;
use std::sync::Arc;
use std::time::Duration;

use once_cell::sync::Lazy;
use toml::Value;

use crate::responses::*;
use crate::structs::*;
use crate::database::validate_sql_table_inputs;
use crate::globals::environment_variables;

use diesel::MysqlConnection;
use diesel::prelude::*;
use diesel::sql_types::*;
use diesel::r2d2::{self, ConnectionManager};

use tokio::net::{TcpListener, TcpStream};
use tokio::sync::{mpsc, Mutex, watch};
use tokio_tungstenite::{accept_async, tungstenite::protocol::Message, tungstenite::protocol::CloseFrame};
use futures_util::{StreamExt, SinkExt};

use elasticsearch::{
    auth::Credentials,
    http::transport::{SingleNodeConnectionPool, Transport, TransportBuilder},
    params::Refresh,
    Elasticsearch, IndexParts, SearchParts,
    cert::CertificateValidation
};

use url::Url;

// Create a type alias for the connection pool
type Pool = r2d2::Pool<ConnectionManager<MysqlConnection>>;

// Create a Lazy static variable for the connection pool
static DB_POOL: Lazy<Pool> = Lazy::new(|| {
    let manager = ConnectionManager::<MysqlConnection>::new(crate::database::get_default_database_url());
    r2d2::Pool::builder()
    .connection_timeout(Duration::from_secs(1))
    .build(manager)
    .expect("Failed to create pool.")
});

pub static CONFIG_VALUE: Lazy<Config> = Lazy::new(|| {
    get_config().expect("Failed to get config")
});

fn get_config() -> Result<Config, String> {
    let environment_variable = "trafficlight_config";
    let mut config_str: String = String::new();
    if let Some(val) = env::var(environment_variable).ok() {
        println!("Value of {}: {}", environment_variable, val);

        config_str = val;
    } else {
        return Err(format!("Missing \"{}\" environment variable", environment_variable).into());
    }

    let config_value: Value = toml::from_str(&config_str).unwrap();
    let config: Config = serde_json::from_value(serde_json::to_value(config_value).expect("Failed to convert config value from toml to serde::json")).expect("Failed to parse config");

    Ok(config)
}

#[catch(500)]
fn internal_error() -> serde_json::Value {
    error_message("server_error", "Internal server error")
}

async fn rocket() -> Rocket<Build> {
    let figment = rocket::Config::figment();

    rocket::custom(figment)
        .attach(Cors)
        .attach(diesel_mysql::stage())
        .register("/", catchers![internal_error])
}

#[tokio::main]
async fn main() {
    env_logger::init();

    log::info!("Starting (Rocket) webserver...");

    tokio::spawn(async {
        loop {
            println!("Running job queue...");
            crate::misc::job::queue().await;
            println!("Finished job queue");

            // Sleep for 1 second. TODO: in the future, this should be until the next present cursor is needed.
            tokio::time::sleep(time::Duration::from_secs(1)).await;
        }
    });

    rocket().await.launch().await.expect("Failed to start web server");
}