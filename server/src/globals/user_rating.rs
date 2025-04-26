use diesel::prelude::*;
use diesel::sql_types::*;
use diesel::sql_query;
use diesel::r2d2::{PooledConnection, ConnectionManager};

use crate::global::{is_null_or_whitespace, request_authentication};
use crate::responses::*;
use crate::structs::*;
use crate::tables::*;

pub async fn get(mut db: PooledConnection<ConnectionManager<MysqlConnection>>, ids: Vec<String>, project_id: String, me: Option<String>) -> (PooledConnection<ConnectionManager<MysqlConnection>>, Vec<User_rating_item>, Option<Vec<User_rating_public>>) {
    let filters = crate::tables::user_rating::id.eq_any(ids.clone())
        .and(crate::tables::user_rating::project.eq(project_id.clone()));

    let filters_me = crate::tables::user_rating::id.eq_any(ids.clone())
        .and(crate::tables::user_rating::author.eq(me.clone().unwrap_or(String::new())))
        .and(crate::tables::user_rating::project.eq(project_id.clone()));

    let user_rating_result: Vec<(Option<String>, String, i64)> = crate::tables::user_rating::table
    .filter(filters.clone())
    .group_by(crate::tables::user_rating::emoji)
    .select((
        diesel::dsl::max(crate::tables::user_rating::id),
        crate::tables::user_rating::emoji,
        diesel::dsl::count_star(),
    ))
    .order(user_rating::created.desc())
    .load(&mut db)
    .expect("Failed to count events");

    let mut user_rating_counts: Vec<User_rating_item> = Vec::new();
    for (id, emoji, count) in user_rating_result {
        user_rating_counts.push(User_rating_item {
            id: id.unwrap(),
            emoji: emoji,
            count: count
        });
    }

    let mut me_user_rating: Option<Vec<User_rating_public>> = None;
    if (me.is_none() == false) {
        let me_user_rating_result: Vec<(User_rating, Option<Accounts>)> = user_rating::table
        .filter(filters_me.clone())
        .left_join(crate::tables::accounts::dsl::accounts.on(crate::tables::user_rating::dsl::author.nullable().eq(crate::tables::accounts::dsl::id.nullable())))
        .order(user_rating::created.asc())
        .select((
            crate::tables::user_rating::all_columns,
            crate::tables::accounts::all_columns.nullable(),
        ))
        .load::<(User_rating, Option<Accounts>)>(&mut *db)
        .expect("Something went wrong querying the DB.");

        let me_user_rating_public: Vec<User_rating_public> = me_user_rating_result
        .into_iter()
        .map(|(user_rating, namespace)| {
            User_rating_public::from((user_rating, namespace.unwrap()))
        })
        .collect();

        me_user_rating = Some(me_user_rating_public);
    }

    return (db, user_rating_counts, me_user_rating);
}