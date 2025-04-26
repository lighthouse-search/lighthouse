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
    discussions (discussion) {
        message_id -> Varchar,
        discussion -> Varchar,
        author -> Varchar,
        content -> Nullable<Text>,
        attachments -> Nullable<Text>,
        created -> Nullable<BigInt>,
        project -> Varchar,
        nonce -> Nullable<Text>
    }
}

diesel::table! {
    user_rating (id) {
        id -> Varchar,
        emoji -> Text,
        author -> Varchar,
        project -> Varchar,
        created -> Nullable<BigInt>
    }
}

diesel::table! {
    issue (id) {
        id -> Varchar,
        title -> Nullable<Text>,
        created -> Nullable<BigInt>,
        project -> Varchar,
        discussion -> Nullable<Varchar>,
    }
}

diesel::table! {
    bug (id) {
        id -> Varchar,
        author -> Varchar,
        title -> Nullable<Text>,
        created -> Nullable<BigInt>,
    }
}

diesel::table! {
    project (id) {
        id -> Varchar,
        namespace -> Varchar,
        name -> Nullable<Text>,
        icon -> Nullable<Text>,
        created -> Nullable<BigInt>,
    }
}

diesel::table! {
    orgs (id) {
        id -> Varchar,
        name -> Nullable<Text>,
        icon -> Nullable<Text>,
        created -> Nullable<BigInt>,
    }
}

diesel::table! {
    namespaces (id) {
        id -> Varchar,
        org -> Varchar,
        name -> Nullable<Text>,
        icon -> Nullable<Text>,
        created -> Nullable<BigInt>,
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

diesel::allow_tables_to_appear_in_same_query!(accounts, discussions, issue, bug, project, orgs, namespaces, device, user_rating);