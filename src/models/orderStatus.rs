use diesel_derive_enum::DbEnum;

#[derive(DbEnum, Debug, Serialize, Deserialize, PartialEq, Clone)]
#[DieselType = "order_status"]
#[serde(rename_all = "snake_case")]
pub enum OrderStatus {
    Pending,
    Paid,
    Preparing,
    Ready,
    Completed,
    Cancelled,
}