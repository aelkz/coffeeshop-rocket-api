use chrono::NaiveDateTime;
use crate::schema::drinks;

use rust_decimal::Decimal;
use serde::{Serialize, Deserialize};

#[derive(Queryable, Insertable, Serialize, Deserialize, Debug)]
#[diesel(table_name = drinks)]
pub struct Drink {
    pub id: String, // store UUIDs as TEXT in SQLite
    pub name: String,
    pub base_price: Decimal,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub deleted_at: Option<NaiveDateTime>
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = drinks)]
pub struct NewDrink<'a> {
    pub id: &'a str,
    pub name: &'a str,
    pub base_price: Decimal
}