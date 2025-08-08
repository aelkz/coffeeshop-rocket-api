/// Drink Catalog Routes
/// 
/// This module provides REST API endpoints for drink catalog operations:
/// - GET /drinks - List all available drinks
/// - GET /drinks/{id} - Get drink by ID
/// - POST /drinks - Create new drink
/// - PUT /drinks/{id} - Update drink price (name is immutable)
/// - DELETE /drinks/{id} - Soft delete drink (future implementation)

use rocket::serde::json::Json;
use rocket::{get, post, put, routes, Route};
use rocket::http::Status;
use diesel::prelude::*;
use uuid::Uuid;
use rust_decimal::Decimal;

use crate::DbConn;
use crate::models::drink::{Drink, DrinkApiModel, NewDrink, UpdateDrink};
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
/// Returns 400 if drink_id is empty.
/// Returns 404 if drink not found or is soft-deleted.
#[get("/drinks/<drink_id>")]
pub async fn get_drink(conn: DbConn, drink_id: String) -> Result<Json<DrinkApiModel>, Status> {
    // Validate drink_id is not empty
    if drink_id.trim().is_empty() {
        eprintln!("Validation error: drink_id cannot be empty");
        return Err(Status::BadRequest);
    }

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
/// Returns 400 if name is empty or price is negative/zero.
/// Returns the created drink with all fields populated.
/// Note: created_at and updated_at are handled automatically server-side.
#[post("/drinks", data = "<new_drink>")]
pub async fn create_drink(
    conn: DbConn,
    new_drink: Json<NewDrink>,
) -> Result<Json<DrinkApiModel>, Status> {
    // Validate input fields
    let drink_data = new_drink.into_inner();
    
    if drink_data.name.trim().is_empty() {
        eprintln!("Validation error: drink name cannot be empty");
        return Err(Status::BadRequest);
    }
    
    if drink_data.base_price <= Decimal::ZERO {
        eprintln!("Validation error: drink price must be greater than zero");
        return Err(Status::BadRequest);
    }

    conn.run(move |c| {
        // Generate a new UUID for the drink
        let drink_id = Uuid::new_v4().to_string();
        
        // Create database model from input (drink_data already extracted above)
        let db_drink = Drink::from_new(drink_data, drink_id);

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

/// Update drink by ID
/// 
/// Updates an existing drink's price only. Drink names are immutable after creation.
/// Returns 400 if drink_id is empty or if price is negative/zero.
/// Returns 422 if request contains unknown fields (e.g., "name" field is not allowed).
/// Returns 404 if drink not found or is soft-deleted.
/// Note: updated_at is handled automatically server-side, created_at remains unchanged.
#[put("/drinks/<drink_id>", data = "<update_drink>")]
pub async fn update_drink(
    conn: DbConn,
    drink_id: String,
    update_drink: Json<UpdateDrink>,
) -> Result<Json<DrinkApiModel>, Status> {
    // Validate drink_id is not empty
    if drink_id.trim().is_empty() {
        eprintln!("Validation error: drink_id cannot be empty");
        return Err(Status::BadRequest);
    }

    // Validate input fields
    let update_data = update_drink.into_inner();
    
    if update_data.base_price <= Decimal::ZERO {
        eprintln!("Validation error: drink price must be greater than zero");
        return Err(Status::BadRequest);
    }

    conn.run(move |c| {
        // First, find the existing drink
        let mut existing_drink = drinks::table
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

        // Update the drink data
        existing_drink.update_from_input(update_data);

        // Save the updated drink back to the database
        diesel::update(drinks::table.filter(drinks::id.eq(&drink_id)))
            .set((
                drinks::base_price.eq(&existing_drink.base_price),
                drinks::updated_at.eq(&existing_drink.updated_at),
            ))
            .execute(c)
            .map_err(|e| {
                eprintln!("Database error updating drink {}: {}", drink_id, e);
                Status::InternalServerError
            })?;

        // Return the updated drink
        Ok(Json(existing_drink.to_api_model()))
    })
    .await
}

/// Export all drink routes
/// 
/// This function returns all drink-related routes that should be mounted
/// on the Rocket application. Mount these under "/api" prefix.
pub fn routes() -> Vec<Route> {
    routes![get_drinks, get_drink, create_drink, update_drink]
}
