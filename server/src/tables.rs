use diesel::prelude::*;
use rocket::serde::{Serialize, Deserialize};
use diesel::r2d2::{self, ConnectionManager};

type DbPool = r2d2::Pool<ConnectionManager<MysqlConnection>>;

diesel::table! {
    posts (id) {
        id -> Nullable<BigInt>,
        title -> Text,
        text -> Text,
        published -> Bool,
    }
}
diesel::table! {
    user (id) {
        id -> Text,
        first_name -> Nullable<Text>,
        last_name -> Nullable<Text>,
        email -> Nullable<Text>,
        permission -> Nullable<BigInt>,
    }
}

diesel::table! {
    magiclink (code) {
        attempt_id -> Text,
        code -> Nullable<BigInt>,
        created -> Nullable<BigInt>,
        attempts -> Nullable<BigInt>,
        user_id -> Text,
    }
}

diesel::table! {
    device (id) {
        id -> Text,
        user_id -> Text,
        public_key -> Text,
        created -> Nullable<BigInt>,
        active -> Nullable<Bool>,
        alias -> Nullable<Text>
    }
}

diesel::table! {
    mindmap_folder (id) {
        id -> Text,
        title -> Nullable<Text>,
        owner -> Nullable<Text>,
        created -> Nullable<BigInt>,
        visibility -> Nullable<Text>,
    }
}

diesel::table! {
    mindmap_item (id) {
        id -> Text,
        title -> Nullable<Text>,
        folder -> Nullable<Text>,
        owner -> Nullable<Text>,
        created -> Nullable<BigInt>,
        visibility -> Nullable<Text>,
    }
}

diesel::table! {
    mindmap_item_content (row_id) {
        row_id -> Text,
        parent -> Nullable<Text>,
        item -> Nullable<Text>,
        rank -> Nullable<BigInt>,
        content -> Nullable<Text>,
        created -> Nullable<BigInt>
    }
}

diesel::table! {
    #[sql_name = "keyword"]
    keywords (keyword) {
        keyword -> Text,
        owner -> Nullable<Text>,
        item -> Nullable<Text>,
        created -> Nullable<BigInt>,
        keyword_metadata -> Text,
    }
}

diesel::table! {
    keyword_metadata (id) {
        id -> Text,
        owner -> Nullable<Text>,
        description -> Nullable<Text>,
        external_link -> Nullable<Text>,
        image -> Nullable<Text>,
        external_image -> Nullable<Text>,
        item -> Nullable<Text>,
        created -> Nullable<BigInt>
    }
}

diesel::table! {
    keyword_sql (id) {
        id -> Nullable<Text>,
        keywords -> Nullable<Text>,
        description -> Nullable<Text>,
        image -> Nullable<Text>,
        external_image -> Nullable<Text>,
        external_link -> Nullable<Text>,
        owner -> Nullable<Text>,
        created -> Nullable<BigInt>
    }
}

diesel::table! {
    rover_processes (device_id) {
        device_id -> Text,
        process -> Text,
        last_seen ->  Nullable<BigInt>,
        user -> Text,
        admin_user -> Text,
        is_admin_process -> Text,
        PID -> Nullable<BigInt>,
        publisher -> Text,
        hash -> Text,
        threads ->  Nullable<BigInt>,
        size ->  Nullable<BigInt>,
        pathname -> Text,
    }
}

diesel::allow_tables_to_appear_in_same_query!(keyword_metadata, keywords);