/// Drink Catalog Routes
/// 
/// This module provides REST API endpoints for drink catalog operations:
/// - GET /drinks - List all available drinks
/// - GET /drinks/{id} - Get drink by ID
/// - POST /drinks - Create new drink
/// - PUT /drinks/{id} - Update drink (future implementation)
/// - DELETE /drinks/{id} - Soft delete drink (future implementation)

use rocket::serde::json::Json;
use rocket::{get, post, routes, Route};
use rocket::http::Status;
use diesel::prelude::*;
use uuid::Uuid;

use crate::DbConn;
use crate::models::drink::{Drink, DrinkApiModel, NewDrink};
use crate::schema::drinks;

/// Get all drinks
/// 
/// Returns a list of all available drinks (not soft-deleted).
/// This endpoint supports filtering and pagination in future iterations.
#[get("/drinks")]
pub async fn get_drinks(conn: DbConn) -> Result<Json<Vec<DrinkApiModel>>, Status> {
    conn.run(|c| {
        // Query all drinks where deleted_at is NULL (available drinks only)
        let results = drinks::table
            .filter(drinks::deleted_at.is_null())
            .select(Drink::as_select())
            .load(c)
            .map_err(|e| {
                eprintln!("Database error loading drinks: {}", e);
                Status::InternalServerError
            })?;

        // Convert database models to API models
        let api_drinks: Vec<DrinkApiModel> = results
            .into_iter()
            .map(|drink| drink.to_api_model())
            .collect();

        Ok(Json(api_drinks))
    })
    .await
}

/// Get drink by ID
/// 
/// Returns a single drink by its unique ID.
/// Returns 404 if drink not found or is soft-deleted.
#[get("/drinks/<drink_id>")]
pub async fn get_drink(conn: DbConn, drink_id: String) -> Result<Json<DrinkApiModel>, Status> {
    conn.run(move |c| {
        let drink = drinks::table
            .filter(drinks::id.eq(&drink_id))
            .filter(drinks::deleted_at.is_null())
            .select(Drink::as_select())
            .first(c)
            .map_err(|e| match e {
                diesel::result::Error::NotFound => Status::NotFound,
                _ => {
                    eprintln!("Database error loading drink {}: {}", drink_id, e);
                    Status::InternalServerError
                }
            })?;

        Ok(Json(drink.to_api_model()))
    })
    .await
}

/// Create new drink
/// 
/// Creates a new drink with the provided information.
/// Generates a UUID for the drink ID automatically.
/// Returns the created drink with all fields populated.
#[post("/drinks", data = "<new_drink>")]
pub async fn create_drink(
    conn: DbConn,
    new_drink: Json<NewDrink>,
) -> Result<Json<DrinkApiModel>, Status> {
    conn.run(move |c| {
        // Generate a new UUID for the drink
        let drink_id = Uuid::new_v4().to_string();
        
        // Create database model from input
        let db_drink = Drink::from_new(new_drink.into_inner(), drink_id);

        // Insert into database
        diesel::insert_into(drinks::table)
            .values(&db_drink)
            .execute(c)
            .map_err(|e| {
                eprintln!("Database error creating drink: {}", e);
                Status::InternalServerError
            })?;

        // Return the created drink
        Ok(Json(db_drink.to_api_model()))
    })
    .await
}

/// Export all drink routes
/// 
/// This function returns all drink-related routes that should be mounted
/// on the Rocket application. Mount these under "/api" prefix.
pub fn routes() -> Vec<Route> {
    routes![get_drinks, get_drink, create_drink]
}
