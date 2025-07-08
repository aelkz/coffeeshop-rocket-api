use crate::schema::orders;
use serde::{Serialize, Deserialize};

#[derive(Queryable, Insertable, Serialize, Deserialize, Debug)]
#[diesel(table_name = orders)]
pub struct Order {
    pub id: String, // store UUIDs as TEXT in SQLite
    pub customer_id: String, // store UUIDs as TEXT in SQLite
    pub employee_id: String, // store UUIDs as TEXT in SQLite
    pub status: OrderStatus, // stored as TEXT
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

/*
    This allows to:
	•	Deserialize a full order request
	•	Break it into:
	•	NewOrder
	•	NewOrderItem
	•	NewOrderItemExtra
 */

#[derive(Deserialize)]
pub struct IncomingOrder {
    pub customer_id: String,
    pub employee_id: String,
    pub status: OrderStatus,
    pub items: Vec<IncomingOrderItem>,
}

#[derive(Deserialize)]
pub struct IncomingOrderItem {
    pub drink_id: String,
    pub size: DrinkSize,
    pub total_price: Decimal, // drink base_price + all extras base_price's
    pub extras: Vec<String>, // list of extra_id strings
}