/// Customer Management Routes
/// 
/// This module provides REST API endpoints for customer operations:
/// - GET /customers - List all customers
/// - GET /customers/{id} - Get customer by ID
/// - POST /customers - Create new customer
/// - PUT /customers/{id} - Update customer (future implementation)
/// - DELETE /customers/{id} - Soft delete customer (future implementation)

use rocket::serde::json::Json;
use rocket::{get, post, routes, Route};
use rocket::http::Status;
use diesel::prelude::*;
use uuid::Uuid;

use crate::DbConn;
use crate::models::customer::{Customer, CustomerApiModel, NewCustomer};
use crate::schema::customers;

/// Get all customers
/// 
/// Returns a list of all active customers (not soft-deleted).
/// This endpoint supports pagination in future iterations.
#[get("/customers")]
pub async fn get_customers(conn: DbConn) -> Result<Json<Vec<CustomerApiModel>>, Status> {
    conn.run(|c| {
        // Query all customers where deleted_at is NULL (active customers only)
        let results = customers::table
            .filter(customers::deleted_at.is_null())
            .select(Customer::as_select())
            .load(c)
            .map_err(|e| {
                eprintln!("Database error loading customers: {}", e);
                Status::InternalServerError
            })?;

        // Convert database models to API models
        let api_customers: Vec<CustomerApiModel> = results
            .into_iter()
            .map(|customer| customer.to_api_model())
            .collect();

        Ok(Json(api_customers))
    })
    .await
}

/// Get customer by ID
/// 
/// Returns a single customer by their unique ID.
/// Returns 404 if customer not found or is soft-deleted.
#[get("/customers/<customer_id>")]
pub async fn get_customer(conn: DbConn, customer_id: String) -> Result<Json<CustomerApiModel>, Status> {
    conn.run(move |c| {
        let customer = customers::table
            .filter(customers::id.eq(&customer_id))
            .filter(customers::deleted_at.is_null())
            .select(Customer::as_select())
            .first(c)
            .map_err(|e| match e {
                diesel::result::Error::NotFound => Status::NotFound,
                _ => {
                    eprintln!("Database error loading customer {}: {}", customer_id, e);
                    Status::InternalServerError
                }
            })?;

        Ok(Json(customer.to_api_model()))
    })
    .await
}

/// Create new customer
/// 
/// Creates a new customer with the provided information.
/// Generates a UUID for the customer ID automatically.
/// Returns the created customer with all fields populated.
#[post("/customers", data = "<new_customer>")]
pub async fn create_customer(
    conn: DbConn,
    new_customer: Json<NewCustomer>,
) -> Result<Json<CustomerApiModel>, Status> {
    conn.run(move |c| {
        // Generate a new UUID for the customer
        let customer_id = Uuid::new_v4().to_string();
        
        // Create database model from input
        let db_customer = Customer::from_new(new_customer.into_inner(), customer_id);

        // Insert into database
        diesel::insert_into(customers::table)
            .values(&db_customer)
            .execute(c)
            .map_err(|e| {
                eprintln!("Database error creating customer: {}", e);
                // Check for unique constraint violation (email)
                if e.to_string().contains("UNIQUE constraint failed") {
                    Status::Conflict
                } else {
                    Status::InternalServerError
                }
            })?;

        // Return the created customer
        Ok(Json(db_customer.to_api_model()))
    })
    .await
}

/// Export all customer routes
/// 
/// This function returns all customer-related routes that should be mounted
/// on the Rocket application. Mount these under "/api" prefix.
pub fn routes() -> Vec<Route> {
    routes![get_customers, get_customer, create_customer]
}
