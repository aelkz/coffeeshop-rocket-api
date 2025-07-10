use diesel_derive_enum::DbEnum;
use serde::{Deserialize, Serialize};

#[derive(DbEnum, Debug, Serialize, Deserialize, PartialEq, Clone)]
#[DieselType = "drink_size"]
#[serde(rename_all = "lowercase")]
pub enum DrinkSize {
    Small,
    Medium,
    Large,
    Standard
}