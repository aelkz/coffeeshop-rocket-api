use rocket_sync_db_pools::{database, diesel};

#[database("sqlite")]
pub struct DbConn(pub diesel::SqliteConnection);