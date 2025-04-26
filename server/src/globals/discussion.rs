use diesel::prelude::*;
use diesel::sql_types::*;
use diesel::sql_query;
use diesel::r2d2::{PooledConnection, ConnectionManager};

use crate::global::{is_null_or_whitespace, request_authentication};
use crate::responses::*;
use crate::structs::*;
use crate::tables::*;

pub async fn get(mut db: PooledConnection<ConnectionManager<MysqlConnection>>, id: Option<String>, nonce: Option<Vec<String>>, project_id: String) -> (PooledConnection<ConnectionManager<MysqlConnection>>, Vec<Discussions_public>, std::collections::HashMap<String, i64>, Vec<Accounts_admin>) {
    let minimum_filter = crate::tables::discussions::project.eq(project_id.clone());

    // Here we just use String::new() and Vec::new() because if the variable (id, nonce) doesn't exist then the filter won't get applied and it doesn't matter if the variable is empty, but the spawned filter still wants a value so we need to put a dummy one in.
    let filters_id = crate::tables::discussions::discussion.eq(id.clone().unwrap_or(String::new()));
    println!("THING: {:?}", nonce.clone().unwrap_or(Vec::new()));
    let filters_nonce = crate::tables::discussions::nonce.eq_any(nonce.clone().unwrap_or(Vec::new()));

    // // TODO: Add permissions, some users won't be able to access things like error events.
    // if (filter.is_none() == false) {
    //     let filter_unwrapped: Discussion_list_filter = serde_json::from_str(&filter.unwrap()).expect("Failed to parse filter.");
        
    //     // TODO: Might be a good idea to run a type check here.
    //     if (filter_unwrapped.nonce.is_none() == false) {
    //         filters = filters.and(crate::tables::discussions::nonce.eq_any(filter_unwrapped.nonce.unwrap()));
    //     }
    // }

    let mut discussions_query = discussions::table
    .filter(minimum_filter.clone())
    .left_join(crate::tables::accounts::dsl::accounts.on(crate::tables::discussions::dsl::author.nullable().eq(crate::tables::accounts::dsl::id.nullable())))
    .order(discussions::created.asc())
    .select((
        crate::tables::discussions::all_columns,
        crate::tables::accounts::all_columns.nullable(),
    ))
    .into_boxed();

    println!("id {:?}", id.clone());
    println!("nonce {:?}", nonce.clone());
    println!("is_none: {}", nonce.is_none());

    if (is_null_or_whitespace(id.clone()) == false) {
        discussions_query = discussions_query.filter(filters_id.clone());
    } 
    if (nonce.is_none() == false) {
        discussions_query = discussions_query.filter(filters_nonce.clone());
    }

    let discussions_result: Vec<(Discussions, Option<Accounts>)> = discussions_query
    .load::<(Discussions, Option<Accounts>)>(&mut *db)
    .expect("Something went wrong querying the DB.");

    let mut discussions_public: Vec<Discussions_public> = discussions_result
    .into_iter()
    .map(|(discussion, account)| {
        Discussions_public::from((discussion, account.unwrap()))
    })
    .collect();

    let mut discussion_counts_query = crate::tables::discussions::table
    .filter(minimum_filter.clone())
    .group_by(crate::tables::discussions::discussion)
    .select((crate::tables::discussions::discussion, diesel::dsl::count_star()))
    .into_boxed();

    if (is_null_or_whitespace(id.clone()) == false) {
        discussion_counts_query = discussion_counts_query.filter(filters_id.clone());
    }
    if (nonce.is_none() == false) {
        discussion_counts_query = discussion_counts_query.filter(filters_nonce.clone());
    }

    let discussion_counts: Vec<(String, i64)> = discussion_counts_query
    .load(&mut db)
    .expect("Failed to count events");

    let mut discussion_counts_map = std::collections::HashMap::new();
    for (id, count) in discussion_counts {
        discussion_counts_map.insert(id, count);
    }

    let mut discussions_distinct_authors_query = discussions::table
    .filter(minimum_filter.clone())
    .left_join(crate::tables::accounts::dsl::accounts.on(crate::tables::discussions::dsl::author.nullable().eq(crate::tables::accounts::dsl::id.nullable())))
    .order(discussions::created.asc())
    .select(crate::tables::accounts::all_columns.nullable())
    .distinct()
    .into_boxed();

    if (is_null_or_whitespace(id.clone()) == false) {
        discussions_distinct_authors_query = discussions_distinct_authors_query.filter(filters_id.clone());
    }
    if (nonce.is_none() == false) {
        discussions_distinct_authors_query = discussions_distinct_authors_query.filter(filters_nonce.clone());
    }

    let discussions_distinct_authors: Vec<Option<Accounts>> = discussions_distinct_authors_query
    .load::<Option<Accounts>>(&mut *db)
    .expect("Something went wrong querying the DB.");

    let mut discussions_distinct_authors_public: Vec<Accounts_admin> = discussions_distinct_authors
    .into_iter()
    .map(|(account)| {
        Accounts_admin::from((account.unwrap()))
    })
    .collect();

    return (db, discussions_public, discussion_counts_map, discussions_distinct_authors_public);
}