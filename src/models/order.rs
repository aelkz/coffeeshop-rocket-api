#[allow(dead_code)]  // suppress warnings for structs used in routes

use crate::schema::orders;
use crate::models::infra::sqlite_types::{SqliteDateTime, SqliteOrderStatus};
use crate::models::order_status::OrderStatus;
use chrono::NaiveDateTime;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use diesel::prelude::*;
use crate::models::drink_size::DrinkSize;

// Database model
#[derive(Queryable, Insertable, Debug)]
#[diesel(table_name = orders)]
pub struct Order {
    pub id: String,
    pub customer_id: String,
    pub employee_id: String,
    pub status: SqliteOrderStatus,
    pub created_at: SqliteDateTime,
    pub updated_at: SqliteDateTime,
}

// API representation
#[derive(Debug, Serialize, Deserialize)]
pub struct OrderApiModel {
    pub id: String,
    pub customer_id: String,
    pub employee_id: String,
    pub status: OrderStatus,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

// Input model for creating orders
#[derive(Debug, Deserialize)]
pub struct NewOrder {
    pub customer_id: String,
    pub employee_id: String,
    pub status: OrderStatus,
}

impl Order {
    pub fn to_api_model(&self) -> OrderApiModel {
        OrderApiModel {
            id: self.id.clone(),
            customer_id: self.customer_id.clone(),
            employee_id: self.employee_id.clone(),
            status: self.status.into_order_status(),
            created_at: self.created_at.into_naive_date_time(),
            updated_at: self.updated_at.into_naive_date_time(),
        }
    }

    pub fn from_new(new: NewOrder, id: String) -> Self {
        let now = chrono::Utc::now().naive_utc();
        Order {
            id,
            customer_id: new.customer_id,
            employee_id: new.employee_id,
            status: SqliteOrderStatus::from(new.status),
            created_at: SqliteDateTime::from(now),
            updated_at: SqliteDateTime::from(now),
        }
    }
}

// Input models for complex order creation
#[derive(Debug, Deserialize)]
pub struct IncomingOrder {
    pub customer_id: String,
    pub employee_id: String,
    pub status: OrderStatus,
    pub items: Vec<IncomingOrderItem>,
}

#[derive(Debug, Deserialize)]
pub struct IncomingOrderItem {
    pub drink_id: String,
    pub size: DrinkSize,
    #[serde(with = "rust_decimal::serde::str")]
    pub total_price: Decimal,
    pub extras: Vec<String>, // List of extra IDs
}

// Conversion for query results
impl From<Order> for OrderApiModel {
    fn from(order: Order) -> Self {
        order.to_api_model()
    }
}

/*
Creating a New Order - Full Example

use rocket::{post, State};
use rocket::serde::json::Json;
use crate::DbConn;
use crate::models::order::{IncomingOrder, Order, NewOrder, OrderApiModel};
use crate::models::order_item::{OrderItem, NewOrderItem};
use crate::models::order_item_extra::{OrderItemExtra, NewOrderItemExtra};

#[post("/orders", data = "<incoming_order>")]
async fn create_order(
    conn: DbConn,
    incoming_order: Json<IncomingOrder>,
) -> Result<Json<OrderApiModel>, Status> {
    // Start transaction
    conn.run(|c| {
        let transaction_result: Result<_, diesel::result::Error> = c.transaction(|tx| {
            // Create the main order
            let order_id = Uuid::new_v4().to_string();
            let db_order = Order::from_new(
                NewOrder {
                    customer_id: incoming_order.customer_id.clone(),
                    employee_id: incoming_order.employee_id.clone(),
                    status: incoming_order.status,
                },
                order_id.clone()
            );

            diesel::insert_into(orders::table)
                .values(&db_order)
                .execute(tx)?;

            // Create order items
            for (item_index, item) in incoming_order.items.iter().enumerate() {
                let item_id = Uuid::new_v4().to_string();
                let db_item = OrderItem::from_new(
                    NewOrderItem {
                        order_id: order_id.clone(),
                        drink_id: item.drink_id.clone(),
                        size: item.size,
                        total_price: item.total_price,
                    },
                    item_id.clone()
                );

                diesel::insert_into(order_items::table)
                    .values(&db_item)
                    .execute(tx)?;

                // Create item extras
                for extra_id in &item.extras {
                    let extra_item_id = Uuid::new_v4().to_string();
                    let db_extra = OrderItemExtra::from_new(
                        NewOrderItemExtra {
                            order_item_id: item_id.clone(),
                            extra_id: extra_id.clone(),
                        },
                        extra_item_id
                    );

                    diesel::insert_into(order_item_extras::table)
                        .values(&db_extra)
                        .execute(tx)?;
                }
            }

            Ok(db_order)
        });

        match transaction_result {
            Ok(order) => Ok(Json(order.to_api_model())),
            Err(e) => {
                error!("Order creation failed: {}", e);
                Err(Status::InternalServerError)
            }
        }
    })
    .await
}
 */