use crate::schema::order_items;
use crate::models::infra::sqlite_types::{SqliteDecimal, SqliteDrinkSize};
use crate::models::drink_size::DrinkSize;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use diesel::prelude::*;

// Database model
#[derive(Queryable, Insertable, Debug)]
#[diesel(table_name = order_items)]
pub struct OrderItem {
    pub id: String,
    pub order_id: String,
    pub drink_id: String,
    pub size: SqliteDrinkSize,
    pub total_price: SqliteDecimal,
}

// API representation
#[derive(Debug, Serialize, Deserialize)]
pub struct OrderItemApiModel {
    pub id: String,
    pub order_id: String,
    pub drink_id: String,
    pub size: DrinkSize,
    #[serde(with = "rust_decimal::serde::str")]
    pub total_price: Decimal,
}

// Input model
#[derive(Debug, Deserialize)]
pub struct NewOrderItem {
    pub order_id: String,
    pub drink_id: String,
    pub size: DrinkSize,
    #[serde(with = "rust_decimal::serde::str")]
    pub total_price: Decimal,
}

impl OrderItem {
    pub fn to_api_model(&self) -> OrderItemApiModel {
        OrderItemApiModel {
            id: self.id.clone(),
            order_id: self.order_id.clone(),
            drink_id: self.drink_id.clone(),
            size: self.size.into_drink_size(),
            total_price: self.total_price.into_decimal(),
        }
    }

    pub fn from_new(new: NewOrderItem, id: String) -> Self {
        OrderItem {
            id,
            order_id: new.order_id,
            drink_id: new.drink_id,
            size: SqliteDrinkSize::from(new.size),
            total_price: SqliteDecimal::from(new.total_price),
        }
    }
}

impl From<OrderItem> for OrderItemApiModel {
    fn from(item: OrderItem) -> Self {
        item.to_api_model()
    }
}