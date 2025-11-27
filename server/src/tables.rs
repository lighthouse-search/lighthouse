use serde::{Serialize, Deserialize};
use serde_json::{Value, json};
use rocket::serde::json::Json;

use diesel::prelude::*;
use diesel::sql_types::*;
use crate::structs::*;

diesel::table! {
    accounts (id) {
        id -> Varchar,
        name -> Nullable<Text>,
        username -> Nullable<Text>,
        email -> Nullable<Text>,
        profile_pic -> Nullable<Text>,
        pronouns -> Nullable<Text>,
        created -> Nullable<BigInt>,
        locked -> Nullable<Bool>,
        suspended -> Nullable<Bool>
    }
}

diesel::table! {
    crawler_queue (id) {
        id -> BigInt,
        url -> Nullable<Text>, // https://user:password@example.com:443/page
        referrer -> Nullable<Text>,
        status -> Nullable<Text>, // considering, pending, crawling, failed
        crawling_node -> Nullable<Text>,
        crawling_since -> Nullable<BigInt>,
        created -> BigInt
    }
}

diesel::table! {
    device (id) {
        id -> Varchar,
        account_id -> Varchar,
        name -> Nullable<Text>,
        public_key -> Text,
        created -> BigInt,
    }
}

diesel::allow_tables_to_appear_in_same_query!(accounts, crawler_queue, device);