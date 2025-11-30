// #[cfg(test)] mod tests;

mod diesel_mysql;
mod global;
mod structs;
mod responses;
mod tables;
mod database;
mod security;
// mod guard;

pub mod globals {
    pub mod environment_variables;
    pub mod text;
}

pub mod endpoint {
    pub mod account;
    pub mod query;
    pub mod crawler;
    // pub mod metadata;
    pub mod admin {
        pub mod index;
    }
    pub mod misc;
}

pub mod network {
    pub mod port;
}

use diesel_mysql::Cors;
// use guard::{build_guard_hostname_to_use, start_guard};
use rocket::fairing::{Fairing, Info, Kind};
use rocket::http::Header;
use rocket::{catch, catchers, Build, Rocket};
use rocket::{Request, Response, request, request::FromRequest};

use std::error::Error;
use std::fs;
use std::collections::HashMap;
use std::env;
use std::path::PathBuf;
use std::sync::Arc;

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

pub static ES: Lazy<Elasticsearch> = Lazy::new(|| {
    // TODO: These environment variables are temporary to stop passwords leaking to Github (even if it's just credentials for the Elastic instance running on my laptop). In the future, these environment variable names will be put into a config file, and code will fetch the environment variable identified in the config.
    let credentials = Credentials::Basic(environment_variables::get("elastic_username").expect("Missing 'elastic_username' env variable."), environment_variables::get("elastic_password").expect("Missing 'elastic_password' env variable."));
    let u = Url::parse(&environment_variables::get("elastic_host").expect("Missing 'elastic_host' env variable.")).expect("Failed to parse url");
    let conn_pool = SingleNodeConnectionPool::new(u);
    let transport = TransportBuilder::new(conn_pool)
        .auth(credentials)
        .cert_validation(CertificateValidation::None)
        .build()
        .expect("Failed to build transport.");
    let client = Elasticsearch::new(transport);

    client
});

pub static CHANNEL: Lazy<(Mutex<mpsc::UnboundedSender<Message>>, Mutex<mpsc::UnboundedReceiver<Message>>)> = Lazy::new(|| {
    let (tx, rx) = mpsc::unbounded_channel(); // Use tokio's mpsc::channel instead of std::sync
    (Mutex::new(tx), Mutex::new(rx))
});

// Create a type alias for the connection pool
type Pool = r2d2::Pool<ConnectionManager<MysqlConnection>>;

// Create a Lazy static variable for the connection pool
static DB_POOL: Lazy<Pool> = Lazy::new(|| {
    let manager = ConnectionManager::<MysqlConnection>::new(crate::database::get_default_database_url());
    r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.")
});

pub static CONFIG_VALUE: Lazy<Config> = Lazy::new(|| {
    get_config().expect("Failed to get config")
});

fn get_config() -> Result<Config, String> {
    let environment_variable = "lighthouse_config";
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

// pub static GUARD_HOSTNAME_TO_USE: Lazy<Guard_hostname_to_use> = Lazy::new(|| {
//     build_guard_hostname_to_use().expect("build_guard_hostname_to_use() failed")
// });

#[catch(500)]
fn internal_error() -> serde_json::Value {
    error_message("server_error", "Internal server error")
}

async fn rocket() -> Rocket<Build> {
    let figment = rocket::Config::figment();

    println!("PORT");
    rocket::custom(figment)
        .attach(Cors)
        .attach(diesel_mysql::stage())
        .register("/", catchers![internal_error])
}

#[tokio::main]
async fn main() {
    env_logger::init();

    // if (GUARD_HOSTNAME_TO_USE.use_local_guard == true) {
    //     GUARD_HOSTNAME_TO_USE.local_port.expect("Missing GUARD_HOSTNAME_TO_USE.local_port");
    //     start_guard(GUARD_HOSTNAME_TO_USE.local_port.unwrap()).await;
    // }

    log::info!("Starting (Rocket) webserver...");
    rocket().await.launch().await.expect("Failed to start web server");
}