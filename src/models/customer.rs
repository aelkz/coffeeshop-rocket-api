use chrono::NaiveDateTime;
use crate::schema::customers;
use serde::{Serialize, Deserialize};

#[derive(Queryable, Insertable, Serialize, Deserialize, Debug)]
#[diesel(table_name = customers)]
pub struct Customer {
    pub id: String, // store UUIDs as TEXT in SQLite
    pub name: String,
    pub email: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub deleted_at: Option<NaiveDateTime>,
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = customers)]
pub struct NewCustomer<'a> {
    pub id: &'a str,
    pub name: &'a str,
    pub email: &'a str,
}