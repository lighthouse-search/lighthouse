#[macro_use] extern crate rocket;

// #[cfg(test)] mod tests;
pub struct Cors;

mod diesel_mysql;
mod global;
mod structs;
mod responses;
mod tables;
mod database;
mod security;

pub mod globals {
    pub mod environment_variables;
    pub mod user_rating;
    pub mod discussion;
}

pub mod endpoint {
    pub mod account;
    pub mod discussion;
    pub mod query;
    pub mod namespace;
    pub mod org;
    pub mod crawler;
    pub mod user_rating;
    // pub mod process;
    pub mod admin {
        pub mod index;
    }
}

pub mod websocket {
    // pub mod connection;
    pub mod event;
}

use rocket::fairing::{Fairing, Info, Kind};
use rocket::http::Header;
use rocket::{Request, Response, request, request::FromRequest};
// use websocket::connection::handle_connection;

use std::error::Error;
use std::fs;
use std::collections::HashMap;
use std::env;
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

// pub static ES: Lazy<Elasticsearch> = Lazy::new(|| {
//     // TODO: These environment variables are temporary to stop passwords leaking to Github (even if it's just credentials for the Elastic instance running on my laptop). In the future, these environment variable names will be put into a config file, and code will fetch the environment variable identified in the config.
//     let credentials = Credentials::Basic(environment_variables::get("elastic_username").expect("Missing 'elastic_username' env variable."), environment_variables::get("elastic_password").expect("Missing 'elastic_password' env variable."));
//     let u = Url::parse(&environment_variables::get("elastic_host").expect("Missing 'elastic_host' env variable.")).expect("Failed to parse url");
//     let conn_pool = SingleNodeConnectionPool::new(u);
//     let transport = TransportBuilder::new(conn_pool).auth(credentials).build().expect("Failed to build transport.");
//     let client = Elasticsearch::new(transport);

//     client
// });

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

pub static CONFIG_VALUE: Lazy<Value> = Lazy::new(|| {
    get_config().expect("Failed to get config")
});

fn get_config() -> Result<Value, Box<dyn Error>> {
    let mut config_value: String = String::new();
    if let Some(val) = env::var("coastguard_config").ok() {
        println!("Value of coastguard_config: {}", val);

        config_value = val;
    } else {
        return Err("Missing \"coastguard_config\" environment variable".into());
    }

    let config: Value = toml::from_str(&config_value).unwrap();

    Ok(config)
}

#[catch(500)]
fn internal_error() -> serde_json::Value {
    error_message("server_error", "Internal server error")
}

#[launch]
async fn rocket() -> _ {
    // Bind the TCP listener to the address
    let addr = "127.0.0.1:8080".to_string();
    let listener = TcpListener::bind(&addr).await.expect("Failed to bind");

    println!("Listening on: {}", addr);

    // tokio::spawn(async move {
    //     // Accept incoming connections
    //     while let Ok((stream, _)) = listener.accept().await {
    //         tokio::spawn(handle_connection(stream));
    //     }
    // });
    
    let figment = rocket::Config::figment();

    rocket::custom(figment)
        .attach(Cors)
        .attach(diesel_mysql::stage())
        .register("/", catchers![internal_error])
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