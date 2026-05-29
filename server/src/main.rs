// // #[cfg(test)] mod tests;

// // The project uses `Capitalized_snake_case` struct names as a deliberate
// // convention, and keeps scaffolding structs/helpers for in-progress endpoints
// // (websockets, guard, etc.). Allow both crate-wide rather than churn every
// // call site or delete planned API surface.
// #![allow(non_camel_case_types, dead_code)]

mod diesel_mysql;
mod global;
mod structs;
mod responses;
mod tables;
mod database;
mod search;
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

pub mod crawl {
    pub mod queue;
}

use std::env;

use once_cell::sync::Lazy;
use toml::Value;

use crate::network::port::find_available_port;
use crate::structs::*;

use diesel::MysqlConnection;
use diesel::r2d2::{self, ConnectionManager};

use tokio::sync::{mpsc, Mutex};
use tokio_tungstenite::tungstenite::protocol::Message;

use tower_http::catch_panic::CatchPanicLayer;

// Re-exported as `crate::ES` so existing call sites
// (`use crate::ES; ES.search(...)`) keep working unchanged.
pub use crate::search::CLIENT as ES;

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
    let config_str: String = match env::var(environment_variable).ok() {
        Some(val) => {
            println!("Value of {}: {}", environment_variable, val);
            val
        }
        None => {
            return Err(format!("Missing \"{}\" environment variable", environment_variable).into());
        }
    };

    let config_value: Value = toml::from_str(&config_str).unwrap();
    let config: Config = serde_json::from_value(serde_json::to_value(config_value).expect("Failed to convert config value from toml to serde::json")).expect("Failed to parse config");

    Ok(config)
}

// pub static GUARD_HOSTNAME_TO_USE: Lazy<Guard_hostname_to_use> = Lazy::new(|| {
//     build_guard_hostname_to_use().expect("build_guard_hostname_to_use() failed")
// });

#[tokio::main]
async fn main() {
    env_logger::init();

    // if (GUARD_HOSTNAME_TO_USE.use_local_guard == true) {
    //     GUARD_HOSTNAME_TO_USE.local_port.expect("Missing GUARD_HOSTNAME_TO_USE.local_port");
    //     start_guard(GUARD_HOSTNAME_TO_USE.local_port.unwrap()).await;
    // }

    // Promote `considering` URLs to `pending` so the crawl queue API has work
    // to hand out.
    std::thread::spawn(crate::crawl::queue::consider_queue);

    let app = diesel_mysql::router()
        .layer(axum::middleware::from_fn(diesel_mysql::cors_middleware))
        .layer(CatchPanicLayer::custom(diesel_mysql::handle_panic));

    let port: u16 = env::var("lighthouse_port")
        .ok()
        .and_then(|p| p.parse().ok())
        .unwrap_or(find_available_port().expect("Failed to find port"));
    let addr = format!("0.0.0.0:{}", port);

    let listener = tokio::net::TcpListener::bind(&addr)
        .await
        .expect("Failed to bind web server");

    log::info!("Starting (axum) webserver on {}...", addr);
    axum::serve(listener, app)
        .await
        .expect("Failed to start web server");
}