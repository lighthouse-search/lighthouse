use serde::{Serialize, Deserialize};
use serde_json::{Value, json};
use rocket::serde::json::Json;

pub fn error_message(error_type: &str, message: &str) -> Value {
    return json!({
        "error": true,
        "type": error_type,
        "message": message.to_string()
    })
}

pub fn not_found(message: &str) -> Value {
    return json!({
        "error": true,
        "message": message.to_string(),
        "not_found": true
    })
}

pub fn not_authorized() -> Value {
    return json!({
        "error": true,
        "message": "Authentication failed (you must authenticate).",
        "unauthorized": true
    })
}