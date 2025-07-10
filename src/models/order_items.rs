use crate::schema::order_items;
use rust_decimal::Decimal;
use serde::{Serialize, Deserialize};
use crate::models::drink_size::DrinkSize;

#[derive(Queryable, Insertable, Serialize, Deserialize, Debug)]
#[diesel(table_name = order_items)]
pub struct OrderItem {
    pub id: String,
    pub order_id: String,
    pub drink_id: String,
    pub size: DrinkSize,
    pub total_price: Decimal,
}