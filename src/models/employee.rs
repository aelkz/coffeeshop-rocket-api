use crate::schema::employees;
use crate::models::infra::sqlite_types::{SqliteDate, SqliteDateTime};
use chrono::{NaiveDate, NaiveDateTime};
use serde::{Deserialize, Serialize};
use diesel::prelude::*;

// Database model (used for querying and inserting)
#[derive(Queryable, Insertable, Selectable, Debug)]
#[diesel(table_name = employees)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Employee {
    pub id: String,
    pub name: String,
    pub email: String,
    pub birth_date: SqliteDate,
    pub created_at: SqliteDateTime,
    pub updated_at: SqliteDateTime,
    pub deleted_at: Option<SqliteDateTime>,
}

// API representation (for serialization/deserialization)
#[derive(Debug, Serialize, Deserialize)]
pub struct EmployeeApiModel {
    pub id: String,
    pub name: String,
    pub email: String,
    /// Birth date in ISO 8601 format (YYYY-MM-DD)
    #[serde(with = "crate::models::infra::sqlite_types::date_format")]
    pub birth_date: NaiveDate,
    #[serde(with = "crate::models::infra::sqlite_types::datetime_format")]
    pub created_at: NaiveDateTime,
    #[serde(with = "crate::models::infra::sqlite_types::datetime_format")]
    pub updated_at: NaiveDateTime,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default, with = "crate::models::infra::sqlite_types::datetime_format_option")]
    pub deleted_at: Option<NaiveDateTime>,
}

// Input model (for creating employees)
#[derive(Debug, Deserialize)]
pub struct NewEmployee {
    pub name: String,
    pub email: String,
    /// Birth date in ISO 8601 format (YYYY-MM-DD)
    /// Must be a valid past date and employee must be of legal working age
    #[serde(with = "crate::models::infra::sqlite_types::date_format")]
    pub birth_date: NaiveDate,
}

// Input model (for updating employees)
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct UpdateEmployee {
    pub name: String,
    pub email: String,
}

impl Employee {
    /// Convert to API-friendly model
    pub fn to_api_model(&self) -> EmployeeApiModel {
        EmployeeApiModel {
            id: self.id.clone(),
            name: self.name.clone(),
            email: self.email.clone(),
            birth_date: self.birth_date.into_naive_date(),
            created_at: self.created_at.into_naive_date_time(),
            updated_at: self.updated_at.into_naive_date_time(),
            deleted_at: self.deleted_at.map(|dt| dt.into_naive_date_time()),
        }
    }

    /// Create a new Employee from input data
    pub fn from_new(new: NewEmployee, id: String) -> Self {
        let now = chrono::Utc::now().naive_utc();
        Employee {
            id,
            name: new.name,
            email: new.email,
            birth_date: SqliteDate::from(new.birth_date),
            created_at: SqliteDateTime::from(now),
            updated_at: SqliteDateTime::from(now),
            deleted_at: None,
        }
    }

    /// Update an existing employee with new data
    /// Only updates name, email, and updated_at. Birth date is immutable, created_at remains unchanged.
    pub fn update_from_input(&mut self, update_data: UpdateEmployee) {
        self.name = update_data.name;
        self.email = update_data.email;
        self.updated_at = SqliteDateTime::from(chrono::Utc::now().naive_utc());
    }
}

// Conversion for query results
impl From<Employee> for EmployeeApiModel {
    fn from(employee: Employee) -> Self {
        employee.to_api_model()
    }
}

/*
// Example usage in route handler
#[post("/employees", data = "<new_employee>")]
async fn create_employee(
    conn: DbConn,
    new_employee: Json<NewEmployee>,
) -> Result<Json<EmployeeApiModel>, Status> {
    let id = Uuid::new_v4().to_string();
    let db_employee = Employee::from_new(new_employee.into_inner(), id);

    conn.run(|c| {
        diesel::insert_into(employees::table)
            .values(&db_employee)
            .execute(c)
            .map_err(|_| Status::InternalServerError)?;

        Ok(Json(db_employee.to_api_model()))
    })
    .await

 */