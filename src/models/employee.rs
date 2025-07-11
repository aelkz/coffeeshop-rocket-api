use crate::schema::employees;
use crate::models::infra::sqlite_types::{SqliteDate, SqliteDateTime};
use chrono::{NaiveDate, NaiveDateTime};
use serde::{Deserialize, Serialize};
use diesel::prelude::*;

// Database model (used for querying and inserting)
#[derive(Queryable, Insertable, Debug)]
#[diesel(table_name = employees)]
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
    pub birth_date: NaiveDate,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub deleted_at: Option<NaiveDateTime>,
}

// Input model (for creating employees)
#[derive(Debug, Deserialize)]
pub struct NewEmployee {
    pub name: String,
    pub email: String,
    pub birth_date: NaiveDate,
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