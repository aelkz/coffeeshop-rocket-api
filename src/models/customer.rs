use crate::schema::customers;
use crate::models::infra::sqlite_types::SqliteDateTime;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use diesel::prelude::*;

// Database model (used for querying and inserting)
#[derive(Queryable, Insertable, Selectable, Debug)]
#[diesel(table_name = customers)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Customer {
    pub id: String,
    pub name: String,
    pub email: String,
    pub created_at: SqliteDateTime,
    pub updated_at: SqliteDateTime,
    pub deleted_at: Option<SqliteDateTime>,
}

// API representation (for serialization/deserialization)
#[derive(Debug, Serialize, Deserialize)]
pub struct CustomerApiModel {
    pub id: String,
    pub name: String,
    pub email: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub deleted_at: Option<NaiveDateTime>,
}

// Input model (for creating customers)
#[derive(Debug, Deserialize)]
pub struct NewCustomer {
    pub name: String,
    pub email: String,
}

// Input model (for updating customers)
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct UpdateCustomer {
    pub name: String,
    pub email: String,
}

impl Customer {
    /// Convert to API-friendly model
    pub fn to_api_model(&self) -> CustomerApiModel {
        CustomerApiModel {
            id: self.id.clone(),
            name: self.name.clone(),
            email: self.email.clone(),
            created_at: self.created_at.into_naive_date_time(),
            updated_at: self.updated_at.into_naive_date_time(),
            deleted_at: self.deleted_at.map(|dt| dt.into_naive_date_time()),
        }
    }

    /// Create a new Customer from input data
    pub fn from_new(new: NewCustomer, id: String) -> Self {
        let now = chrono::Utc::now().naive_utc();
        Customer {
            id,
            name: new.name,
            email: new.email,
            created_at: SqliteDateTime::from(now),
            updated_at: SqliteDateTime::from(now),
            deleted_at: None,
        }
    }
    
    /// Update an existing customer with new data
    /// Only updates name, email, and updated_at. created_at remains unchanged.
    pub fn update_from_input(&mut self, update_data: UpdateCustomer) {
        self.name = update_data.name;
        self.email = update_data.email;
        self.updated_at = SqliteDateTime::from(chrono::Utc::now().naive_utc());
    }
}

// Conversion for query results
impl From<Customer> for CustomerApiModel {
    fn from(customer: Customer) -> Self {
        customer.to_api_model()
    }
}

/*
Usage Example:

// In your route handler
#[post("/customers", data = "<new_customer>")]
async fn create_customer(
    conn: DbConn,
    new_customer: Json<NewCustomer>,
) -> Result<Json<CustomerApiModel>, Status> {
    let id = Uuid::new_v4().to_string();
    let db_customer = Customer::from_new(new_customer.into_inner(), id);

    conn.run(|c| {
        diesel::insert_into(customers::table)
            .values(&db_customer)
            .execute(c)
            .map_err(|_| Status::InternalServerError)?;

        Ok(Json(db_customer.to_api_model()))
    })
    .await
 */
