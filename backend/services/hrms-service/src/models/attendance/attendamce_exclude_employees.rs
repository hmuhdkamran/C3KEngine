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
#[serde(rename_all = "PascalCase")]
pub struct AttendamceExcludeEmployees {
    pub attendamce_exclude_employee_id: Uuid,
pub employee_id: Uuid,
pub reason: String,
pub status_id: Uuid

}

impl AttendamceExcludeEmployees {
    pub const TABLE: &'static str = r#""Attendance"."AttendamceExcludeEmployees""#;
    pub const PK: &'static str = "AttendamceExcludeEmployeeId";
    pub const COLUMNS_ARRAY: [&'static str; 4] = ["AttendamceExcludeEmployeeId","EmployeeId","Reason","StatusId"];

    pub fn get_id(&self) -> Uuid {
        self.attendamce_exclude_employee_id.clone()
    }

    pub fn get_args(&self) -> PgArguments {
        let mut args = PgArguments::default();
        let _ = args.add(self.attendamce_exclude_employee_id.clone());
let _ = args.add(self.employee_id.clone());
let _ = args.add(self.reason.clone());
let _ = args.add(self.status_id.clone());

        args
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
