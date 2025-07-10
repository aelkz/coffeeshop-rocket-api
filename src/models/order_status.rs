use diesel_derive_enum::DbEnum;
use serde::{Deserialize, Serialize};

#[derive(DbEnum, Debug, Serialize, Deserialize, PartialEq, Clone)]
#[DieselType = "order_status"]
#[serde(rename_all = "lowercase")]
pub enum OrderStatus {
    Pending,
    Paid,
    Preparing,
    Ready,
    Completed,
    Cancelled,
}