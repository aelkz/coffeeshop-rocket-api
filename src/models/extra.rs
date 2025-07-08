use crate::schema::extras;
use serde::{Serialize, Deserialize};

#[derive(Queryable, Insertable, Serialize, Deserialize, Debug)]
#[diesel(table_name = extras)]
pub struct Extra {
    pub id: String, // store UUIDs as TEXT in SQLite
    pub name: String,
    pub extra_price: Decimal,
    pub is_available: bool
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = extras)]
pub struct NewExtra<'a> {
    pub name: &'a str,
    pub extra_price: Decimal,
}