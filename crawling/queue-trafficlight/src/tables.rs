use serde::{Serialize, Deserialize};
use serde_json::{Value, json};
use rocket::serde::json::Json;

use diesel::prelude::*;
use diesel::sql_types::*;
use crate::structs::*;

diesel::table! {
    crawler_runner (runner_ip) {
        runner_ip -> Varchar,
        nonce -> Text, // When a runner boots, it generates a unique string. This allows multiple nodes to run behind the same IP. You can also use this to manage amnesia: This string changes every boot and ensures traffic-light knows if the runner has rebooted since being assigned a job. If a runner reboots mid-job, it's likely the job failed and needs to be re-assigned.
        capacity -> Nullable<BigInt>,
    }
}

diesel::table! {
    crawler_jobs (id) {
        id -> Varchar, // Uniquie job identifier.
        runner_ip -> Varchar, // IP address of runner.
        nonce -> Varchar, // Boot nonce of node.
        status -> Nullable<Text>, // e.g. "completed", "running", "stopped", "error"
        cursor_currently -> Nullable<BigInt>, // If a job interupts, we want to know where the runner progressed to, rather than re-indexing the entire range.
        cursor_from -> Nullable<BigInt>, // Cursor to start from.
        cursor_to -> Nullable<BigInt> // Cursor to end with.
    }
}

diesel::table! {
    crawler_missingranges (missing_from) {
        missing_from -> BigInt,
        missing_to -> BigInt
    }
}

diesel::allow_tables_to_appear_in_same_query!(runner, jobs, missing_ranges);