use diesel::prelude::*;

use std::process::{Command, Stdio};
use std::error::Error;
use std::collections::HashMap;
use std::fs::{File};
use std::io::Write;
use std::env;

use crate::CONFIG_VALUE;
use crate::structs::*;
use crate::tables::*;

use url::Url;
use rand::prelude::*;

use lettre::message::header::ContentType;
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};

use hades_auth::{authenticate, static_auth_verify};

pub fn generate_random_id() -> String {
    let mut random_string = String::new();
    const CHARACTERS: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZABCDEFGHIJKLMNOPQRSTUVWXYZ";
    for _ in 0..CHARACTERS.len() {
        let index = rand::thread_rng().gen_range(0..CHARACTERS.len());
        random_string.push(CHARACTERS.chars().nth(index).unwrap());
    }
    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_millis();
    random_string.truncate(20);
    random_string + &timestamp.to_string()
}

pub fn is_null_or_whitespace(data: Option<String>) -> bool {
    if (data.is_none()) {
        return true;
    }
    let s = data.unwrap();
    match s {
        string if string == "null" || string == "undefined" => true,
        string => string.trim().is_empty(),
    }
}

pub async fn request_authentication(body: Option<String>, params: &Query_string, pathname: &str) -> Result<Request_authentication_output, Box<dyn Error>> {
    let mut db = crate::DB_POOL.get().expect("Failed to get a connection from the pool.");

    // Parse request params.
    let mut params_object: HashMap<String, String> = HashMap::new();
    let params_string: String = params.0.clone();
    if !params_string.is_empty() {
        params_object = Url::parse(&format!("http://localhost/?{}", params_string))
        .map(|url| url.query_pairs().into_owned().collect())
        .unwrap_or_default();
    }

    // Check request params include a deviceid.
    if (params_object.get("deviceid").is_none()) {
        // throw an error.
    }

    // TODO: These errors don't show, you could be missing something as basic as a param and it would return an unhelpful 401. need to fix.
    
    // Parse deviceid.
    let device_id = match params_object.get("deviceid") {
        Some(id) => id.clone(),
        None => return Err("Missing deviceid parameter".into()), // Handle missing deviceid gracefully
    };

    // Parse project_id.
    let project_id = match params_object.get("project_id") {
        Some(id) => id.clone(),
        None => return Err("Missing project_id parameter".into()), // Handle missing project_id gracefully
    };

    // Check request params include a authenticator_jwt_token param.
    if (params_object.get("authenticator_JWT_Token").is_none()) {
        // throw an error.
    }
    // Parse JWT token.
    let jwt = match params_object.get("authenticator_JWT_Token") {
        Some(id) => id.clone(),
        None => return Err("Missing authenticator_JWT_Token parameter".into()), // Handle missing deviceid gracefully
    };

    // Query devices table in database for the specific deviceid.
    let result: Option<Device> = device::table
        .filter(device::id.eq(&device_id))
        .first(&mut db)
        .optional()
        .expect("Something went wrong querying the DB1.");

    // Check the device exists.
    if (result.is_none()) {
        return Err("Authentication failed [device doesn't exist]".into())
    }

    // Parse the device results.
    let device = result.unwrap();

    // put relevant data into variables.
    let public_key = device.public_key;
    let account_id = device.account_id;

    // Ensure relevant data is signed.
    authenticate(
        body,
        serde_json::to_value(params_object).unwrap(),
        &jwt,
        &public_key,
        &format!("/api/native-v1{}", pathname),
        false
    ).await.expect("Authentication failed");

    // Authentication is good, return results to internal function.
    return Ok(Request_authentication_output {
        // returned_connection: db,
        device_id: device_id,
        account_id: account_id,
        // TODO: [SECURITY]: CHECK THIS USER CAN AUTHORISE THIS PROJECT_ID.
        project_id: project_id
    });
}

pub async fn request_authentication_staticauth(jwt: Option<&str>, device_id: &str, project_id: &str) -> Result<Request_authentication_output, Box<dyn Error>> {
    let mut db = crate::DB_POOL.get().expect("Failed to get a connection from the pool.");

    let result: Option<Device> = device::table
        .filter(device::id.eq(&device_id))
        .first(&mut db)
        .optional().expect("Something went wrong querying the DB1.");

    if (result.is_none()) {
        return Err("Authentication failed [device doesn't exist]".into())
    }

    let device = result.unwrap();

    let public_key = device.public_key;
    let account_id = device.account_id;

    static_auth_verify(
        &jwt.expect("Missing jwt").to_string(),
        &public_key,
        None
    ).await.expect("Authentication failed");

    println!("Auth didn't fail");

    return Ok(Request_authentication_output {
        // returned_connection: db,
        device_id: device.id,
        account_id: account_id,
        // TODO: [SECURITY]: CHECK THIS USER CAN AUTHORISE THIS PROJECT_ID.
        project_id: project_id.to_string()
    });
}