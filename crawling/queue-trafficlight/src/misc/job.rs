use rocket::response::{status, status::Custom};
use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::{get, post};

use serde::{Serialize, Deserialize};
use serde_json::{Value, json};

use diesel::prelude::*;
use diesel::sql_types::*;
use diesel::sql_query;

use crate::global::{generate_random_id, is_null_or_whitespace};
use crate::responses::*;
use crate::structs::*;
use crate::tables::*;

use std::net::SocketAddr;

pub async fn queue() {
    let mut db = crate::DB_POOL.get().expect("Failed to get a connection from the pool.");

    // Find the missing seq ranges we need to backfill from the relay. Think of this as the work we need to do.
    let missing_ranges = crate::misc::missing_range::list().await;
    println!("missing_ranges {:?}", missing_ranges);
    // Let's find runners available to take jobs.
    let available_runners = crate::misc::runner::list(true, true).await;
    println!("available_runners {:?}", available_runners);

    // Create a loop based jobs, rather than runners to fill. Not only does this make sense, but it allows us to quickly send the job to another runner if there is a faulty runner unable to start.
    for missing_range in missing_ranges {
        // TODO: Missing_range needs to be brought within range and another missing_range object should be inserted as another job. Make the splitting code a function.
        
        log::info!("Assigning job (from: {}) (to: {})", missing_range.missing_from, missing_range.missing_to);
        // We've got a job to do, start from the least busy runner, from the list we got before.
        for slot in &available_runners {
            // Attempt to send job to runner.
            let job_id = generate_random_id();
            let job_status = crate::misc::runner::send_job(job_id.clone(), &slot.runner, &missing_range).await;
            if (job_status.is_ok() == true) {
                // The job was sent successfully, break out of the loop and move to the next job.
                // TODO: need to log this actually happened lmao.

                entry_create(job_id.clone(), &slot.runner, &missing_range, "running").await.expect("Failed to log job entry");

                log::info!("Assigned job (from: {}) (to: {}) to {}", missing_range.missing_from, missing_range.missing_to, slot.runner.runner_ip);
                break;
            } else {
                // The job failed, try the next runner.
                // TODO: Write a function that logs that runner as faulty. Also need to check the runner actually failed and didn't just deny.
                log::error!("Failed to assign job (from: {}) (to: {}) to {}, moving to another runner...", missing_range.missing_from, missing_range.missing_to, slot.runner.runner_ip);
                continue;
            }
        }
    };
}

pub async fn entry_create(job_id: String, runner: &Runner, missing_range: &Missing_ranges, status: &str) -> Result<(), String> {
    let mut db = crate::DB_POOL.get().expect("Failed to get a connection from the pool.");

    // Check if the job already exists.
    let job_exists = diesel::select(diesel::dsl::exists(
        jobs::table
            .filter(jobs::cursor_from.eq(missing_range.missing_from))
            .filter(jobs::cursor_to.eq(missing_range.missing_to))
    ))
    .get_result::<bool>(&mut db)
    .expect("Failed to check if job exists with the specified range");

    if job_exists == true {
        return Err("Job already exists".to_string());
    }

    let job_id_exists = diesel::select(diesel::dsl::exists(
        jobs::table
            .filter(jobs::id.eq(job_id.clone()))
    ))
    .get_result::<bool>(&mut db)
    .expect("Failed to check if job exists with the specified range");

    if job_id_exists == true {
        return Err("Job with specified id already exists".to_string());
    }

    // Create the job.
    let new_job = Jobs {
        id: job_id.clone(),
        status: Some(status.to_string()),
        runner_ip: runner.runner_ip.clone(),
        nonce: runner.nonce.clone(),
        cursor_from: Some(missing_range.missing_from),
        cursor_to: Some(missing_range.missing_to),
        cursor_currently: None
    };

    diesel::insert_into(jobs::table)
        .values(&new_job)
        .execute(&mut db)
        .expect("Failed to create job");

    Ok(())
}

pub async fn entry_update(job_id: String, runner: &Runner, missing_range: &Missing_ranges, status: &str, cursor_currently: &u64) -> Result<(), String> {
    let mut db = crate::DB_POOL.get().expect("Failed to get a connection from the pool.");

    // Check if the job exists, we don't want to be chasing ghosts.
    let job_exists = diesel::select(diesel::dsl::exists(
        jobs::table
            .filter(jobs::id.eq(job_id.clone()))
            .filter(jobs::runner_ip.eq(runner.runner_ip.clone()))
    ))
    .get_result::<bool>(&mut db)
    .expect("Failed to check if job exists for the specific runner");

    if job_exists == false {
        return Err("Job doesn't exist for the specified runner".to_string());
    }

    diesel::update(crate::tables::jobs::table)
    .filter(crate::tables::jobs::id.eq(job_id.clone()))
    .filter(crate::tables::jobs::runner_ip.eq(runner.runner_ip.clone()))
    .set((jobs::status.eq(status), jobs::cursor_currently.eq(cursor_currently.clone() as i64)))
    .execute(&mut db)
    .expect("Failed to update");

    Ok(())
}