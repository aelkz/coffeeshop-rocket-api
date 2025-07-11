use rust_decimal::Decimal;
use chrono::{NaiveDate, NaiveDateTime};
use diesel::{
    sql_types::Text,
    deserialize::{self, FromSql},
    serialize::{self, ToSql, Output},
    sqlite::Sqlite,
    expression::AsExpression,
    deserialize::FromSqlRow,
};
use std::str::FromStr;
use crate::models::drink_size::DrinkSize;
use crate::models::order_status::OrderStatus;

// custom type for Decimal
#[derive(Debug, Clone, Copy, AsExpression, FromSqlRow)]
#[diesel(sql_type = Text)]
pub struct SqliteDecimal(pub Decimal);

impl FromSql<Text, Sqlite> for SqliteDecimal {
    fn from_sql(value: diesel::sqlite::SqliteValue) -> deserialize::Result<Self> {
        let s = <String as FromSql<Text, Sqlite>>::from_sql(value)?;
        Decimal::from_str(&s)
            .map(SqliteDecimal)
            .map_err(|e| e.into())
    }
}

impl ToSql<Text, Sqlite> for SqliteDecimal {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Sqlite>) -> serialize::Result {
        let s = self.0.to_string();
        out.set_value(s);
        Ok(serialize::IsNull::No)
    }
}

impl From<Decimal> for SqliteDecimal {
    fn from(value: Decimal) -> Self {
        SqliteDecimal(value)
    }
}

impl SqliteDecimal {
    pub fn into_decimal(self) -> Decimal {
        self.0
    }
}

// custom type for NaiveDateTime
#[derive(Debug, Clone, Copy, AsExpression, FromSqlRow)]
#[diesel(sql_type = Text)]
pub struct SqliteDateTime(pub NaiveDateTime);

impl FromSql<Text, Sqlite> for SqliteDateTime {
    fn from_sql(value: diesel::sqlite::SqliteValue) -> deserialize::Result<Self> {
        let s = <String as FromSql<Text, Sqlite>>::from_sql(value)?;
        NaiveDateTime::parse_from_str(&s, "%Y-%m-%dT%H:%M:%S%.f")
            .map(SqliteDateTime)
            .map_err(Into::into)
    }
}

impl ToSql<Text, Sqlite> for SqliteDateTime {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Sqlite>) -> serialize::Result {
        out.set_value(self.0.format("%Y-%m-%dT%H:%M:%S%.f").to_string());
        Ok(serialize::IsNull::No)
    }
}

impl From<NaiveDateTime> for SqliteDateTime {
    fn from(value: NaiveDateTime) -> Self {
        SqliteDateTime(value)
    }
}

impl SqliteDateTime {
    pub fn into_naive_date_time(self) -> NaiveDateTime {
        self.0
    }
}

// custom type for NaiveDate
#[derive(Debug, Clone, Copy, AsExpression, FromSqlRow)]
#[diesel(sql_type = Text)]
pub struct SqliteDate(pub NaiveDate);

impl FromSql<Text, Sqlite> for SqliteDate {
    fn from_sql(value: diesel::sqlite::SqliteValue) -> deserialize::Result<Self> {
        let s = <String as FromSql<Text, Sqlite>>::from_sql(value)?;
        NaiveDate::parse_from_str(&s, "%Y-%m-%d")
            .map(SqliteDate)
            .map_err(Into::into)
    }
}

impl ToSql<Text, Sqlite> for SqliteDate {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Sqlite>) -> serialize::Result {
        out.set_value(self.0.format("%Y-%m-%d").to_string());
        Ok(serialize::IsNull::No)
    }
}

impl From<NaiveDate> for SqliteDate {
    fn from(value: NaiveDate) -> Self {
        SqliteDate(value)
    }
}

impl SqliteDate {
    pub fn into_naive_date(self) -> NaiveDate {
        self.0
    }
}

#[derive(Debug, Clone, Copy, AsExpression, FromSqlRow)]
#[diesel(sql_type = Text)]
pub struct SqliteDrinkSize(pub DrinkSize);

impl FromSql<Text, Sqlite> for SqliteDrinkSize {
    fn from_sql(value: diesel::sqlite::SqliteValue) -> deserialize::Result<Self> {
        let s = <String as FromSql<Text, Sqlite>>::from_sql(value)?;
        s.parse()
            .map(SqliteDrinkSize)
            .map_err(|e| e.into())
    }
}

impl ToSql<Text, Sqlite> for SqliteDrinkSize {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Sqlite>) -> serialize::Result {
        out.set_value(self.0.to_string());
        Ok(serialize::IsNull::No)
    }
}

impl From<DrinkSize> for SqliteDrinkSize {
    fn from(value: DrinkSize) -> Self {
        SqliteDrinkSize(value)
    }
}

impl SqliteDrinkSize {
    pub fn into_drink_size(self) -> DrinkSize {
        self.0
    }
}

#[derive(Debug, Clone, Copy, AsExpression, FromSqlRow)]
#[diesel(sql_type = Text)]
pub struct SqliteOrderStatus(pub OrderStatus);

impl FromSql<Text, Sqlite> for SqliteOrderStatus {
    fn from_sql(value: diesel::sqlite::SqliteValue) -> deserialize::Result<Self> {
        let s = <String as FromSql<Text, Sqlite>>::from_sql(value)?;
        s.parse()
            .map(SqliteOrderStatus)
            .map_err(|e| e.into())
    }
}

impl ToSql<Text, Sqlite> for SqliteOrderStatus {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Sqlite>) -> serialize::Result {
        out.set_value(self.0.to_string());
        Ok(serialize::IsNull::No)
    }
}

impl From<OrderStatus> for SqliteOrderStatus {
    fn from(value: OrderStatus) -> Self {
        SqliteOrderStatus(value)
    }
}

impl SqliteOrderStatus {
    pub fn into_order_status(self) -> OrderStatus {
        self.0
    }
}