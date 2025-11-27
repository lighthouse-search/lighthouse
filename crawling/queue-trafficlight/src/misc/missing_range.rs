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
use crate::{responses::*, CONFIG_VALUE};
use crate::structs::*;
use crate::tables::*;

use std::net::SocketAddr;

// Yes, I know:
    // let config = CONFIG_VALUE.clone();
    // let min_cursor = config.cursor.clone().expect("Missing config.cursor").max_backfill.unwrap() as i64 * 1000000;  // Seconds to microseconds
    // let max_range = config.cursor.clone().expect("Missing config.cursor").max_assignable_cursor.unwrap() as i64 / 1000000; // Seconds to microseconds
// should be a function or something. I am thinking about the best way to do it, but for now we have this.

pub async fn list() -> Vec<Missing_ranges> {
    let config = CONFIG_VALUE.clone();
    let min_cursor = config.cursor.clone().expect("Missing config.cursor").max_backfill.unwrap() as i64 * 1000000;  // Seconds to microseconds
    let max_range = config.cursor.clone().expect("Missing config.cursor").max_assignable_cursor.unwrap() as i64 / 1000000; // Seconds to microseconds

    let mut db = crate::DB_POOL.get().expect("Failed to get a connection from the pool.");

    let query = r#"
WITH gaps AS (          -- step 1 ── isolate every missing range
    SELECT
        `cursor` + 1 AS missing_from,
        next_cursor  - 1 AS missing_to
    FROM (
        SELECT
            `cursor`,
            LEAD(`cursor`) OVER (ORDER BY `cursor`) AS next_cursor
        FROM posts
    ) p
    WHERE next_cursor IS NOT NULL          -- skip the final row
      AND next_cursor <> `cursor` + 1      -- signals a break
      AND next_cursor - 1 >= ?      -- Ensure cursor is over certain value, we don't want to backfill posts from years ago if the indexing range is 7 days.
)

SELECT g.*
FROM   gaps g                           -- step 2 ── keep only the gaps
WHERE  NOT EXISTS (                     --   that are *not* contained in a
    SELECT 1                            --   “completed” or “running” job
    FROM   jobs j
    WHERE  j.status       IN ('completed', 'running')
      AND  j.cursor_from <= g.missing_from   -- job starts at/before the gap
      AND  j.cursor_to   >= g.missing_to     -- job ends at/after   the gap
)
ORDER BY g.missing_from;"#;

    let mut build_output: Vec<Missing_ranges> = sql_query(query)
    .bind::<BigInt, _>(min_cursor)
    .load::<Missing_ranges>(&mut db)
    .expect("Something went wrong querying the DB.");

    // The above SQL query only returns gaps between stored cursors (e.g. if Monday was indexed, Tuesday wasn't, Wednesday was), and will not detect gaps to the present (e.g. Monday indexed, Tuesday indexed, Wednesday indexed, present (Thursday) not indexed).
    // We'll push a range from [last recorded gap]-[present cursor], which will likely be outside our cursor range, so we'll throw "output" through "bring_within_range" to pull our output within range.
    if (should_add_present_range(build_output.clone()) == true) { // First off, we need to check we haven't already scheduled an index into the future.
        build_output.push(build_present_range(build_output.clone()));
    }

    return bring_within_range(build_output.clone()).await;
}

pub async fn bring_within_range(missing_ranges: Vec<Missing_ranges>) -> Vec<Missing_ranges> {
    let config = CONFIG_VALUE.clone();
    let min_cursor = config.cursor.clone().expect("Missing config.cursor").max_backfill.unwrap() as i64 * 1000000;  // Seconds to microseconds
    let max_range = config.cursor.clone().expect("Missing config.cursor").max_assignable_cursor.unwrap() as i64 / 1000000; // Seconds to microseconds

    let mut new_ranges = Vec::new();
    
    for missing_range in missing_ranges {
        if (missing_range.missing_to-missing_range.missing_from > max_range) {
            // The range is too big, we need to split it.
            let mut new_range = missing_range.clone();
            // Set this range to the maximum size, move the overflow into another range.

            new_range.missing_to = missing_range.missing_from + max_range;
            new_ranges.push(new_range);
        } else {
            // Range is fine, add it.
            new_ranges.push(missing_range);
        }
    }

    return new_ranges;
}

pub fn build_present_range(build_output: Vec<Missing_ranges>) -> Missing_ranges {
    let config = CONFIG_VALUE.clone();
    let min_cursor = config.cursor.clone().expect("Missing config.cursor").max_backfill.unwrap() as i64 * 1000000;  // Seconds to microseconds
    let max_range = config.cursor.clone().expect("Missing config.cursor").max_assignable_cursor.unwrap() as i64 / 1000000; // Seconds to microseconds
    
    // Put timestamp into variable so it isn't relative. If we call them seperately, we will go outside the max job cursor by a few microseconds.
    let current_timestamp = crate::misc::cursor::unix_microseconds();

    // Check if we can pick up from the last cursor, if we can't, start from current cursor.
    let mut missing_from = current_timestamp;
    if (build_output.len() > 0) {
        missing_from = build_output[build_output.len()].missing_to;
    }
    return Missing_ranges {
        missing_from: missing_from,
        missing_to: current_timestamp+max_range
    }
}

pub fn should_add_present_range(build_output: Vec<Missing_ranges>) -> bool {
    let mut db = crate::DB_POOL.get().expect("Failed to get a connection from the pool.");

    let config = CONFIG_VALUE.clone();
    let min_cursor = config.cursor.clone().expect("Missing config.cursor").max_backfill.unwrap() as i64 * 1000000;  // Seconds to microseconds
    let max_range = config.cursor.clone().expect("Missing config.cursor").max_assignable_cursor.unwrap() as i64 / 1000000; // Seconds to microseconds
    
    // Get the current cursor to check if we're still indexing the present.
    let current_cursor: Option<Jobs> = jobs::table
    .filter(jobs::cursor_to.ge(min_cursor.clone()))
    .first(&mut db)
    .optional()
    .expect("DB error");

    if (current_cursor.is_some() == true) {
        let current_cursor_unwrapped = current_cursor.unwrap();
        if (current_cursor_unwrapped.cursor_to.is_none() == true) {
            // cursor_to is null, treat this as having no present cursor.
            return true;
        }
        if (current_cursor_unwrapped.cursor_to.unwrap() > crate::misc::cursor::unix_microseconds()) {
            // The present cursor is in the future, no need to add a present range.
            return false;
        } else {
            // We currently aren't indexing to the present, add a present range.
            return true;
        }
    } else {
        // No cursor, add a present range.
        return true;
    }
}