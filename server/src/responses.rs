use rocket::serde::json::Value;
use rocket::serde::json::json;

pub fn error_message(message: &str) -> Value {
    return json!({
        "error": true,
        "message": message.to_string()
    })
}

pub fn not_authorized() -> Value {
    return json!({
        "error": true,
        "message": "Authentication failed (you must authenticate).",
        "unauthorized": true
    })
}