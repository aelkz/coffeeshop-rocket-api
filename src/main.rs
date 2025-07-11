#[macro_use] extern crate diesel;
#[macro_use] extern crate rocket;

mod db;
mod models;
mod schema;

use dotenvy::dotenv;
use diesel::result::Error::NotFound;
use rocket::{Rocket, Build};
use rocket::fairing::AdHoc;
use rocket::http::Status;
use rocket::serde::json::{Value, json, Json};
use rocket::response::status::{self, Custom};
use rocket_sync_db_pools::database;

#[database("sqlite")]
struct DbConn(diesel::SqliteConnection);

#[get("/")]
fn hello() -> &'static str {
    "Hello from Rocket!"
}

async fn run_db_migrations(rocket: Rocket<Build>) -> Rocket<Build> {
    use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
    const MIGRATIONS: EmbeddedMigrations = embed_migrations!();

    DbConn::get_one(&rocket)
        .await
        .expect("unable to retrieve connection").run(|c| {
        c.run_pending_migrations(MIGRATIONS).expect("migrations failed");
    }).await;

    rocket
}

#[rocket::main]
async fn main() {
    dotenv().ok(); // loads .env into process environment

    let _ = rocket::build()
        .mount("/", routes![hello])
        .attach(DbConn::fairing())
        .attach(AdHoc::on_ignite("Database Initialization", run_db_migrations))
        .launch()
        .await;
}