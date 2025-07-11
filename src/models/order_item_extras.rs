use crate::schema::order_item_extras;
use serde::{Deserialize, Serialize};
use diesel::prelude::*;

// Database model (used for querying and inserting)
#[derive(Queryable, Insertable, Debug)]
#[diesel(table_name = order_item_extras)]
pub struct OrderItemExtra {
    pub id: String,
    pub order_item_id: String,
    pub extra_id: String,
}

// API representation (for serialization/deserialization)
#[derive(Debug, Serialize, Deserialize)]
pub struct OrderItemExtraApiModel {
    pub id: String,
    pub order_item_id: String,
    pub extra_id: String,
}

// Input model (for creating order item extras)
#[derive(Debug, Deserialize)]
pub struct NewOrderItemExtra {
    pub order_item_id: String,
    pub extra_id: String,
}

impl OrderItemExtra {
    /// Convert to API-friendly model
    pub fn to_api_model(&self) -> OrderItemExtraApiModel {
        OrderItemExtraApiModel {
            id: self.id.clone(),
            order_item_id: self.order_item_id.clone(),
            extra_id: self.extra_id.clone(),
        }
    }

    /// Create a new OrderItemExtra from input data
    pub fn from_new(new: NewOrderItemExtra, id: String) -> Self {
        OrderItemExtra {
            id,
            order_item_id: new.order_item_id,
            extra_id: new.extra_id,
        }
    }
}

// Conversion for query results
impl From<OrderItemExtra> for OrderItemExtraApiModel {
    fn from(extra: OrderItemExtra) -> Self {
        extra.to_api_model()
    }
}

/*
// In your route handler
#[post("/order-items/<order_item_id>/extras", data = "<new_extra>")]
async fn add_extra_to_order_item(
    conn: DbConn,
    order_item_id: String,
    new_extra: Json<NewOrderItemExtra>,
) -> Result<Json<OrderItemExtraApiModel>, Status> {
    let id = Uuid::new_v4().to_string();
    let db_extra = OrderItemExtra::from_new(
        NewOrderItemExtra {
            order_item_id: order_item_id.clone(),
            extra_id: new_extra.extra_id.clone(),
        },
        id
    );
    
    conn.run(|c| {
        diesel::insert_into(order_item_extras::table)
            .values(&db_extra)
            .execute(c)
            .map_err(|_| Status::InternalServerError)?;
            
        Ok(Json(db_extra.to_api_model()))
    })
    .await
}
 */