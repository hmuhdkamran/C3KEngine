use c3k_common::interfaces::irepository::Model;
use serde::{Deserialize, Serialize};
pub use sqlx::{
    pool::PoolConnection,
    postgres::{PgArguments, PgPoolOptions, PgRow},
    Arguments, Error, PgPool, Postgres, Row,
    types::chrono::{DateTime, Utc},
};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AttendamceExcludeEmployees {
    pub attendamce_exclude_employee_id: Uuid,
pub employee_id: Uuid,
pub reason: String,
pub status_id: Uuid

}

impl AttendamceExcludeEmployees {
    pub const TABLE: &'static str = r#""Attendance"."AttendamceExcludeEmployees""#;
    pub const PK: &'static str = r#"AttendamceExcludeEmployeeId::TEXT=$1"#;
    pub const COLUMNS: &'static str = r#""AttendamceExcludeEmployeeId","EmployeeId","Reason","StatusId""#;
    pub const COLUMNS_UPDATE: &'static str = r#""AttendamceExcludeEmployeeId"=$1,"EmployeeId"=$2,"Reason"=$3,"StatusId"=$4 WHERE "AttendamceExcludeEmployeeId"=$1"#;

    pub fn get_id(&self) -> Uuid {
        self.attendamce_exclude_employee_id.clone()
    }

    pub fn new(attendamce_exclude_employee_id: Uuid,employee_id: Uuid,reason: String,status_id: Uuid) -> Self {
        Self {
            attendamce_exclude_employee_id,
employee_id,
reason,
status_id

        }
    }
}

impl PartialEq for AttendamceExcludeEmployees {
    fn eq(&self, other: &Self) -> bool {
        self.attendamce_exclude_employee_id == other.attendamce_exclude_employee_id
    }
}

impl Model for AttendamceExcludeEmployees {
    fn from_row(row: &PgRow) -> AttendamceExcludeEmployees
    where
        Self: Sized,
    {
        let attendamce_exclude_employee_id = row.get("AttendamceExcludeEmployeeId");
let employee_id = row.get("EmployeeId");
let reason = row.get("Reason");
let status_id = row.get("StatusId");


        Self {
            attendamce_exclude_employee_id,
employee_id,
reason,
status_id

        }
    }
}
