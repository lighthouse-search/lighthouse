use diesel::prelude::*;
use diesel::MysqlConnection;

use crate::global::get_timestamp;
use crate::structs::Crawler_queue;
use crate::tables::crawler_queue;

/// Reads the oldest `pending` URL from SQL and atomically claims it for
/// `node_id`, transitioning it `pending` -> `crawling`. The select + update run
/// inside a transaction with a row lock (`FOR UPDATE`) so two crawl nodes can
/// never claim the same URL. Returns `None` when the queue is empty.
pub fn claim_next_url(db: &mut MysqlConnection, node_id: &str) -> Option<Crawler_queue> {
    db.transaction::<_, diesel::result::Error, _>(|conn| {
        let next: Option<Crawler_queue> = crawler_queue::table
            .filter(crawler_queue::status.eq("pending"))
            .order(crawler_queue::created.asc())
            .for_update()
            .first::<Crawler_queue>(conn)
            .optional()?;

        let Some(item) = next else {
            return Ok(None);
        };

        diesel::update(crawler_queue::table.filter(crawler_queue::id.eq(item.id)))
            .set((
                crawler_queue::status.eq("crawling"),
                crawler_queue::crawling_node.eq(node_id),
                crawler_queue::crawling_since.eq(get_timestamp() as i64),
            ))
            .execute(conn)?;

        Ok(Some(item))
    })
    .expect("Failed to claim URL from crawl queue")
}

/// Background worker: promotes URLs that have cleared consideration
/// (`considering` -> `pending`) so the crawl queue API has something to hand
/// out. Runs on its own thread for the lifetime of the process.
pub fn consider_queue() {
    loop {
        std::thread::sleep(std::time::Duration::from_secs(60));

        let mut db = match crate::DB_POOL.get() {
            Ok(db) => db,
            Err(e) => {
                log::error!("consider_queue: failed to get DB connection: {e}");
                continue;
            }
        };

        // TODO: real consideration (robots.txt, per-host rate limits, dedup
        // against already-indexed URLs). For now every considered URL is
        // promoted straight to pending.
        let result = diesel::update(
            crawler_queue::table.filter(crawler_queue::status.eq("considering")),
        )
        .set(crawler_queue::status.eq("pending"))
        .execute(&mut *db);

        match result {
            Ok(n) if n > 0 => log::info!("consider_queue: promoted {n} URL(s) to pending"),
            Ok(_) => {}
            Err(e) => log::error!("consider_queue: promotion failed: {e}"),
        }
    }
}
