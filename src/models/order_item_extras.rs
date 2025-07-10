use crate::schema::order_item_extras;
use serde::{Serialize, Deserialize};

#[derive(Queryable, Insertable, Serialize, Deserialize, Debug)]
#[diesel(table_name = order_item_extras)]
pub struct OrderItemExtra {
    pub id: String,
    pub order_item_id: String,
    pub extra_id: String,
}