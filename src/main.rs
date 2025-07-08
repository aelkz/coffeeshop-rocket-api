mod db;
mod schema;  // ← this makes `crate::schema` available
mod models;

#[macro_use] extern crate diesel;
#[macro_use] extern crate rocket;

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

#[rocket::main]
async fn main() {
    dotenv().ok(); // loads .env into process environment
    // use db::DbConn;
    // use crate::models::customer::Customer;

    let _ = rocket::build()
        .mount("/", routes![hello])
        .attach(DbConn::fairing())
        .launch()
        .await;
}