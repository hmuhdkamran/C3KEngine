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
pub struct OvertimeRequests {
    pub overtime_request_id: Uuid,
pub employee_id: Uuid,
pub overtime_date: DateTime<Utc>,
pub overtime_hours: f64,
pub reason: String,
pub request_date: DateTime<Utc>,
pub status_id: Uuid

}

impl OvertimeRequests {
    pub const TABLE: &'static str = r#""Attendance"."OvertimeRequests""#;
    pub const PK: &'static str = "OvertimeRequestId";
    pub const COLUMNS_ARRAY: [&'static str; 7] = ["OvertimeRequestId","EmployeeId","OvertimeDate","OvertimeHours","Reason","RequestDate","StatusId"];

    pub fn get_id(&self) -> Uuid {
        self.overtime_request_id.clone()
    }

    pub fn get_args(&self) -> PgArguments {
        let mut args = PgArguments::default();
        let _ = args.add(self.overtime_request_id.clone());
let _ = args.add(self.employee_id.clone());
let _ = args.add(self.overtime_date.clone());
let _ = args.add(self.overtime_hours.clone());
let _ = args.add(self.reason.clone());
let _ = args.add(self.request_date.clone());
let _ = args.add(self.status_id.clone());

        args
    }

    pub fn new(overtime_request_id: Uuid,employee_id: Uuid,overtime_date: DateTime<Utc>,overtime_hours: f64,reason: String,request_date: DateTime<Utc>,status_id: Uuid) -> Self {
        Self {
            overtime_request_id,
employee_id,
overtime_date,
overtime_hours,
reason,
request_date,
status_id

        }
    }
}

impl PartialEq for OvertimeRequests {
    fn eq(&self, other: &Self) -> bool {
        self.overtime_request_id == other.overtime_request_id
    }
}

impl Model for OvertimeRequests {
    fn from_row(row: &PgRow) -> OvertimeRequests
    where
        Self: Sized,
    {
        let overtime_request_id = row.get("OvertimeRequestId");
let employee_id = row.get("EmployeeId");
let overtime_date = row.get("OvertimeDate");
let overtime_hours = row.get("OvertimeHours");
let reason = row.get("Reason");
let request_date = row.get("RequestDate");
let status_id = row.get("StatusId");


        Self {
            overtime_request_id,
employee_id,
overtime_date,
overtime_hours,
reason,
request_date,
status_id

        }
    }
}
