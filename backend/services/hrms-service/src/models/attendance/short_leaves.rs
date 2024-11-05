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
pub struct ShortLeaves {
    pub employee_short_leave_id: Uuid,
pub employee_id: Uuid,
pub status_id: Uuid,
pub start_date_time: DateTime<Utc>,
pub end_date_time: DateTime<Utc>,
pub reason: String

}

impl ShortLeaves {
    pub const TABLE: &'static str = r#""Attendance"."ShortLeaves""#;
    pub const PK: &'static str = "EmployeeShortLeaveId";
    pub const COLUMNS_ARRAY: [&'static str; 6] = ["EmployeeShortLeaveId","EmployeeId","StatusId","StartDateTime","EndDateTime","Reason"];

    pub fn get_id(&self) -> Uuid {
        self.employee_short_leave_id.clone()
    }

    pub fn get_args(&self) -> PgArguments {
        let mut args = PgArguments::default();
        let _ = args.add(self.employee_short_leave_id.clone());
let _ = args.add(self.employee_id.clone());
let _ = args.add(self.status_id.clone());
let _ = args.add(self.start_date_time.clone());
let _ = args.add(self.end_date_time.clone());
let _ = args.add(self.reason.clone());

        args
    }

    pub fn new(employee_short_leave_id: Uuid,employee_id: Uuid,status_id: Uuid,start_date_time: DateTime<Utc>,end_date_time: DateTime<Utc>,reason: String) -> Self {
        Self {
            employee_short_leave_id,
employee_id,
status_id,
start_date_time,
end_date_time,
reason

        }
    }
}

impl PartialEq for ShortLeaves {
    fn eq(&self, other: &Self) -> bool {
        self.employee_short_leave_id == other.employee_short_leave_id
    }
}

impl Model for ShortLeaves {
    fn from_row(row: &PgRow) -> ShortLeaves
    where
        Self: Sized,
    {
        let employee_short_leave_id = row.get("EmployeeShortLeaveId");
let employee_id = row.get("EmployeeId");
let status_id = row.get("StatusId");
let start_date_time = row.get("StartDateTime");
let end_date_time = row.get("EndDateTime");
let reason = row.get("Reason");


        Self {
            employee_short_leave_id,
employee_id,
status_id,
start_date_time,
end_date_time,
reason

        }
    }
}
