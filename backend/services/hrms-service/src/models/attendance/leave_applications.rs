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
    pub const PK: &'static str = "LeaveApplicationId";
    pub const COLUMNS_ARRAY: [&'static str; 9] = ["LeaveApplicationId","EmployeeId","LeaveTypeId","StartDate","EndDate","FullHalfDay","StatusId","Reason","ApplicationDate"];

    pub fn get_id(&self) -> Uuid {
        self.leave_application_id.clone()
    }

    pub fn get_args(&self) -> PgArguments {
        let mut args = PgArguments::default();
        let _ = args.add(self.leave_application_id.clone());
let _ = args.add(self.employee_id.clone());
let _ = args.add(self.leave_type_id.clone());
let _ = args.add(self.start_date.clone());
let _ = args.add(self.end_date.clone());
let _ = args.add(self.full_half_day.clone());
let _ = args.add(self.status_id.clone());
let _ = args.add(self.reason.clone());
let _ = args.add(self.application_date.clone());

        args
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
