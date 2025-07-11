use crate::schema::extras;
use crate::models::infra::sqlite_types::SqliteDecimal;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use diesel::prelude::*;

// Database model (used for querying and inserting)
#[derive(Queryable, Insertable, Debug)]
#[diesel(table_name = extras)]
pub struct Extra {
    pub id: String,
    pub name: String,
    pub extra_price: SqliteDecimal,
    pub is_available: bool,
}

// API representation (for serialization/deserialization)
#[derive(Debug, Serialize, Deserialize)]
pub struct ExtraApiModel {
    pub id: String,
    pub name: String,
    #[serde(with = "rust_decimal::serde::str")]
    pub extra_price: Decimal,
    pub is_available: bool,
}

// Input model (for creating extras)
#[derive(Debug, Deserialize)]
pub struct NewExtra {
    pub name: String,
    #[serde(with = "rust_decimal::serde::str")]
    pub extra_price: Decimal,
    pub is_available: Option<bool>, // Optional with default
}

impl Extra {
    /// Convert to API-friendly model
    pub fn to_api_model(&self) -> ExtraApiModel {
        ExtraApiModel {
            id: self.id.clone(),
            name: self.name.clone(),
            extra_price: self.extra_price.into_decimal(),
            is_available: self.is_available,
        }
    }

    /// Create a new Extra from input data
    pub fn from_new(new: NewExtra, id: String) -> Self {
        Extra {
            id,
            name: new.name,
            extra_price: SqliteDecimal::from(new.extra_price),
            is_available: new.is_available.unwrap_or(true), // Default to available
        }
    }
}

// Conversion for query results
impl From<Extra> for ExtraApiModel {
    fn from(extra: Extra) -> Self {
        extra.to_api_model()
    }
}

/*
// Example usage in route handler
#[post("/extras", data = "<new_extra>")]
async fn create_extra(
    conn: DbConn,
    new_extra: Json<NewExtra>,
) -> Result<Json<ExtraApiModel>, Status> {
    let id = Uuid::new_v4().to_string();
    let db_extra = Extra::from_new(new_extra.into_inner(), id);
    
    conn.run(|c| {
        diesel::insert_into(extras::table)
            .values(&db_extra)
            .execute(c)
            .map_err(|_| Status::InternalServerError)?;
            
        Ok(Json(db_extra.to_api_model()))
    })
    .await
}
 */