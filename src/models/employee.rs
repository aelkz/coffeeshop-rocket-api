use chrono::{NaiveDate, NaiveDateTime};
use serde::{Serialize, Deserialize};

#[derive(Queryable, Insertable, Serialize, Deserialize, Debug)]
#[diesel(table_name = employees)]
pub struct Employee {
    pub id: String, // store UUIDs as TEXT in SQLite
    pub name: String,
    pub email: String,
    pub birth_date: NaiveDate,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub deleted_at: Option<NaiveDateTime>,
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = employees)]
pub struct NewEmployee<'a> {
    pub id: &'a str,
    pub name: &'a str,
    pub email: &'a str,
    pub birth_date: NaiveDate
}