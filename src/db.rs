use rocket_sync_db_pools::database;
use diesel::SqliteConnection;

// responsible for defining the DB connection and Rocket pool wrapper.
#[database("sqlite")]
pub struct DbConn(SqliteConnection);