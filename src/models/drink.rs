use crate::schema::drinks;
use crate::models::infra::sqlite_types::{SqliteDecimal, SqliteDateTime};
use chrono::NaiveDateTime;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use diesel::prelude::*;

// database model (used for querying and inserting)
#[derive(Queryable, Insertable, Selectable, Debug)]
#[diesel(table_name = drinks)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Drink {
    pub id: String,
    pub name: String,
    pub base_price: SqliteDecimal,
    pub created_at: SqliteDateTime,
    pub updated_at: SqliteDateTime,
    pub deleted_at: Option<SqliteDateTime>,
}

// API representation (for serialization/deserialization)
#[derive(Debug, Serialize, Deserialize)]
pub struct DrinkApiModel {
    pub id: String,
    pub name: String,
    #[serde(with = "rust_decimal::serde::str")]
    pub base_price: Decimal,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub deleted_at: Option<NaiveDateTime>,
}

// input model (for creating drinks)
#[derive(Debug, Deserialize)]
pub struct NewDrink {
    pub name: String,
    #[serde(with = "rust_decimal::serde::str")]
    pub base_price: Decimal,
}

// input model (for updating drinks)
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct UpdateDrink {
    #[serde(with = "rust_decimal::serde::str")]
    pub base_price: Decimal,
}

impl Drink {
    /// convert to API-friendly model
    pub fn to_api_model(&self) -> DrinkApiModel {
        DrinkApiModel {
            id: self.id.clone(),
            name: self.name.clone(),
            base_price: self.base_price.into_decimal(),
            created_at: self.created_at.into_naive_date_time(),
            updated_at: self.updated_at.into_naive_date_time(),
            deleted_at: self.deleted_at.map(|dt| dt.into_naive_date_time()),
        }
    }

    /// create a new Drink from input data
    pub fn from_new(new: NewDrink, id: String) -> Self {
        let now = chrono::Utc::now().naive_utc();
        Drink {
            id,
            name: new.name,
            base_price: SqliteDecimal::from(new.base_price),
            created_at: SqliteDateTime::from(now),
            updated_at: SqliteDateTime::from(now),
            deleted_at: None,
        }
    }
    
    /// Update an existing drink with new data
    /// Only updates base_price and updated_at. Name is immutable, created_at remains unchanged.
    pub fn update_from_input(&mut self, update_data: UpdateDrink) {
        self.base_price = SqliteDecimal::from(update_data.base_price);
        self.updated_at = SqliteDateTime::from(chrono::Utc::now().naive_utc());
    }
}

// conversion for query results
impl From<Drink> for DrinkApiModel {
    fn from(drink: Drink) -> Self {
        drink.to_api_model()
    }
}

/*
Usage Example:

// In your route handler
#[post("/drinks", data = "<new_drink>")]
async fn create_drink(
    conn: DbConn,
    new_drink: Json<NewDrink>,
) -> Result<Json<DrinkApiModel>, Status> {
    let id = Uuid::new_v4().to_string();
    let db_drink = Drink::from_new(new_drink.into_inner(), id);

    conn.run(|c| {
        diesel::insert_into(drinks::table)
            .values(&db_drink)
            .execute(c)
            .map_err(|_| Status::InternalServerError)?;

        Ok(Json(db_drink.to_api_model()))
    })
    .await
 */