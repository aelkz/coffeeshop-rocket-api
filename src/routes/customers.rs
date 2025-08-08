/// Customer Management Routes
/// 
/// This module provides REST API endpoints for customer operations:
/// - GET /customers - List all customers
/// - GET /customers/{id} - Get customer by ID
/// - POST /customers - Create new customer
/// - PUT /customers/{id} - Update customer
/// - DELETE /customers/{id} - Soft delete customer (future implementation)

use rocket::serde::json::Json;
use rocket::{get, post, put, routes, Route};
use rocket::http::Status;
use diesel::prelude::*;
use uuid::Uuid;

use crate::DbConn;
use crate::models::customer::{Customer, CustomerApiModel, NewCustomer, UpdateCustomer};
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
/// Returns 400 if customer_id is empty.
/// Returns 404 if customer not found or is soft-deleted.
#[get("/customers/<customer_id>")]
pub async fn get_customer(conn: DbConn, customer_id: String) -> Result<Json<CustomerApiModel>, Status> {
    // Validate customer_id is not empty
    if customer_id.trim().is_empty() {
        eprintln!("Validation error: customer_id cannot be empty");
        return Err(Status::BadRequest);
    }

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
/// Returns 400 if name or email is empty.
/// Returns the created customer with all fields populated.
/// Note: created_at and updated_at are handled automatically server-side.
#[post("/customers", data = "<new_customer>")]
pub async fn create_customer(
    conn: DbConn,
    new_customer: Json<NewCustomer>,
) -> Result<Json<CustomerApiModel>, Status> {
    // Validate input fields
    let customer_data = new_customer.into_inner();
    
    if customer_data.name.trim().is_empty() {
        eprintln!("Validation error: customer name cannot be empty");
        return Err(Status::BadRequest);
    }
    
    if customer_data.email.trim().is_empty() {
        eprintln!("Validation error: customer email cannot be empty");
        return Err(Status::BadRequest);
    }
    
    // Basic email validation
    if !customer_data.email.contains('@') {
        eprintln!("Validation error: invalid email format");
        return Err(Status::BadRequest);
    }

    conn.run(move |c| {
        // Generate a new UUID for the customer
        let customer_id = Uuid::new_v4().to_string();
        
        // Create database model from input (customer_data already extracted above)
        let db_customer = Customer::from_new(customer_data, customer_id);

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

/// Update customer by ID
/// 
/// Updates an existing customer with new information.
/// Returns 400 if customer_id is empty or if name/email are empty.
/// Returns 422 if request contains unknown fields (only name and email are allowed).
/// Returns 404 if customer not found or is soft-deleted.
/// Note: updated_at is handled automatically server-side, created_at remains unchanged.
#[put("/customers/<customer_id>", data = "<update_customer>")]
pub async fn update_customer(
    conn: DbConn,
    customer_id: String,
    update_customer: Json<UpdateCustomer>,
) -> Result<Json<CustomerApiModel>, Status> {
    // Validate customer_id is not empty
    if customer_id.trim().is_empty() {
        eprintln!("Validation error: customer_id cannot be empty");
        return Err(Status::BadRequest);
    }

    // Validate input fields
    let update_data = update_customer.into_inner();
    
    if update_data.name.trim().is_empty() {
        eprintln!("Validation error: customer name cannot be empty");
        return Err(Status::BadRequest);
    }
    
    if update_data.email.trim().is_empty() {
        eprintln!("Validation error: customer email cannot be empty");
        return Err(Status::BadRequest);
    }
    
    // Basic email validation
    if !update_data.email.contains('@') {
        eprintln!("Validation error: invalid email format");
        return Err(Status::BadRequest);
    }

    conn.run(move |c| {
        // First, find the existing customer
        let mut existing_customer = customers::table
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

        // Update the customer data
        existing_customer.update_from_input(update_data);

        // Save the updated customer back to the database
        diesel::update(customers::table.filter(customers::id.eq(&customer_id)))
            .set((
                customers::name.eq(&existing_customer.name),
                customers::email.eq(&existing_customer.email),
                customers::updated_at.eq(&existing_customer.updated_at),
            ))
            .execute(c)
            .map_err(|e| {
                eprintln!("Database error updating customer {}: {}", customer_id, e);
                // Check for unique constraint violation (email)
                if e.to_string().contains("UNIQUE constraint failed") {
                    Status::Conflict
                } else {
                    Status::InternalServerError
                }
            })?;

        // Return the updated customer
        Ok(Json(existing_customer.to_api_model()))
    })
    .await
}

/// Export all customer routes
/// 
/// This function returns all customer-related routes that should be mounted
/// on the Rocket application. Mount these under "/api" prefix.
pub fn routes() -> Vec<Route> {
    routes![get_customers, get_customer, create_customer, update_customer]
}
