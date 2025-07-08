use diesel_derive_enum::DbEnum;

#[derive(DbEnum, Debug, Serialize, Deserialize, PartialEq, Clone)]
#[DieselType = "drink_size"]
#[serde(rename_all = "snake_case")]
pub enum DrinkSize {
    Small,
    Medium,
    Large,
    Standard
}