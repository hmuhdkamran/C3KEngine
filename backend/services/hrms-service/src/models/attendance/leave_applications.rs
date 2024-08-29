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
pub struct LeaveApplications {
    pub leave_application_id: Uuid,
pub employee_id: Uuid,
pub leave_type_id: Uuid,
pub start_date: DateTime<Utc>,
pub end_date: DateTime<Utc>,
pub full_half_day: bool,
pub status_id: Uuid,
pub reason: String,
pub application_date: DateTime<Utc>

}

impl LeaveApplications {
    pub const TABLE: &'static str = r#""Attendance"."LeaveApplications""#;
    pub const PK: &'static str = r#"LeaveApplicationId::TEXT=$1"#;
    pub const COLUMNS: &'static str = r#""LeaveApplicationId","EmployeeId","LeaveTypeId","StartDate","EndDate","FullHalfDay","StatusId","Reason","ApplicationDate""#;
    pub const COLUMNS_UPDATE: &'static str = r#""LeaveApplicationId"=$1,"EmployeeId"=$2,"LeaveTypeId"=$3,"StartDate"=$4,"EndDate"=$5,"FullHalfDay"=$6,"StatusId"=$7,"Reason"=$8,"ApplicationDate"=$9 WHERE "LeaveApplicationId"=$1"#;

    pub fn get_id(&self) -> Uuid {
        self.leave_application_id.clone()
    }

    pub fn new(leave_application_id: Uuid,employee_id: Uuid,leave_type_id: Uuid,start_date: DateTime<Utc>,end_date: DateTime<Utc>,full_half_day: bool,status_id: Uuid,reason: String,application_date: DateTime<Utc>) -> Self {
        Self {
            leave_application_id,
employee_id,
leave_type_id,
start_date,
end_date,
full_half_day,
status_id,
reason,
application_date

        }
    }
}

impl PartialEq for LeaveApplications {
    fn eq(&self, other: &Self) -> bool {
        self.leave_application_id == other.leave_application_id
    }
}

impl Model for LeaveApplications {
    fn from_row(row: &PgRow) -> LeaveApplications
    where
        Self: Sized,
    {
        let leave_application_id = row.get("LeaveApplicationId");
let employee_id = row.get("EmployeeId");
let leave_type_id = row.get("LeaveTypeId");
let start_date = row.get("StartDate");
let end_date = row.get("EndDate");
let full_half_day = row.get("FullHalfDay");
let status_id = row.get("StatusId");
let reason = row.get("Reason");
let application_date = row.get("ApplicationDate");


        Self {
            leave_application_id,
employee_id,
leave_type_id,
start_date,
end_date,
full_half_day,
status_id,
reason,
application_date

        }
    }
}
