#[macro_use] extern crate diesel;
#[macro_use] extern crate rocket;

mod db;
mod models;
mod routes;
mod schema;

use dotenvy::dotenv;
use rocket::{Rocket, Build, Request, catch, catchers};
use rocket::fairing::AdHoc;
use rocket::http::Status;
use rocket::serde::json::{Value, json, Json};
use rocket_sync_db_pools::database;

#[database("sqlite")]
struct DbConn(diesel::SqliteConnection);

/// Health check endpoint
/// 
/// Returns a simple greeting to verify the API is running.
/// This endpoint doesn't require database access.
#[get("/")]
fn hello() -> &'static str {
    "Coffee Shop API is running!"
}

/// JSON error catcher for 400 Bad Request
#[catch(400)]
fn bad_request(_req: &Request) -> Json<Value> {
    Json(json!({
        "error": "Bad Request",
        "message": "The request was invalid or cannot be served."
    }))
}

/// JSON error catcher for 404 Not Found
#[catch(404)]
fn not_found(_req: &Request) -> Json<Value> {
    Json(json!({
        "error": "Not Found",
        "message": "The requested resource could not be found."
    }))
}

/// JSON error catcher for 422 Unprocessable Entity
#[catch(422)]
fn unprocessable_entity(_req: &Request) -> Json<Value> {
    Json(json!({
        "error": "Unprocessable Entity",
        "message": "The request was well-formed but contains invalid data or unknown fields."
    }))
}

/// JSON error catcher for 500 Internal Server Error
#[catch(500)]
fn internal_error(_req: &Request) -> Json<Value> {
    Json(json!({
        "error": "Internal Server Error", 
        "message": "An unexpected error occurred while processing the request."
    }))
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
        // Health check endpoint
        .mount("/", routes![hello])
        
        // API endpoints - all mounted under /api prefix
        .mount("/api", routes::customers::routes())  // /api/customers/*
        .mount("/api", routes::drinks::routes())     // /api/drinks/*
        
        // Database connection pool
        .attach(DbConn::fairing())
        
        // Run database migrations on startup
        .attach(AdHoc::on_ignite("Database Initialization", run_db_migrations))
        
        // Register JSON error catchers
        .register("/", catchers![bad_request, not_found, unprocessable_entity, internal_error])
        
        .launch()
        .await;
}