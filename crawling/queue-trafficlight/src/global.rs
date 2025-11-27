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