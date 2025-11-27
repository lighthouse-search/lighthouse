use rocket::response::{status, status::Custom};
use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::{get, post};

use serde::{Serialize, Deserialize};
use serde_json::{Value, json};

use diesel::prelude::*;
use diesel::sql_types::*;
use diesel::sql_query;

use crate::global::is_null_or_whitespace;
use crate::responses::*;
use crate::structs::*;
use crate::tables::*;

use std::collections::HashMap;
use std::net::SocketAddr;
use diesel::dsl::{count_star};

pub async fn list(available: bool, least_usage: bool) -> Vec<Runner_available> {
    let mut db = crate::DB_POOL.get().expect("Failed to get a connection from the pool.");

    use diesel::prelude::*;

    let output_sql = runner::table
    .left_join(jobs::table.on(
        runner::runner_ip
            .eq(jobs::runner_ip)
            .and(runner::nonce.eq(jobs::nonce)),
    ))
    .select((
        runner::all_columns,
        diesel::dsl::sql::<diesel::sql_types::BigInt>("COUNT(jobs.id) as job_count"),
    ))
    .group_by((runner::runner_ip, runner::nonce))
    .load::<(Runner, i64)>(&mut *db)
    .expect("Something went wrong querying the DB.");

    let mut output: Vec<Runner_available> = output_sql
    .into_iter()
    .filter(|(_, job_count)| {
        if available == false {
            // Caller doesn't care if the runner is available or not.
            return true;
        }

        if available == true && *job_count < 3 {
            // Caller cares if runner is available 
            return true;
        } else {
            // Runner is not available because it's outside the job limit.
            return false;
        }
    })
    .map(|(runner, job_count)| Runner_available {
        runner,
        job_count,
    })
    .collect();

    if least_usage == true {
        // Caller wants to sort by the least used runners.
        output.sort_by_key(|runner| runner.job_count);
    }

    return output;
}

pub async fn send_job(job_id: String, runner: &Runner, missing_range: &Missing_ranges) -> Result<(), String> {
    let mut map = HashMap::new();
    map.insert("job_id", job_id);
    map.insert("cursor_from", missing_range.missing_from.to_string());
    map.insert("cursor_to", missing_range.missing_to.to_string());

    let mut url = url::Url::parse(&format!("http://{}", runner.runner_ip)).unwrap();
    url.set_path("/internal/traffic-light/job/create");
    url.set_port(Some(3000)).expect("Failed to set port");

    let client = reqwest::Client::new();
    let res = client.post(url.as_str())
        .json(&map)
        .send()
        .await.expect("Failed to contact runner");

    Ok(())
}