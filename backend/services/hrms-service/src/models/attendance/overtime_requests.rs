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
    pub const PK: &'static str = r#"OvertimeRequestId::TEXT=$1"#;
    pub const COLUMNS: &'static str = r#""OvertimeRequestId","EmployeeId","OvertimeDate","OvertimeHours","Reason","RequestDate","StatusId""#;
    pub const COLUMNS_UPDATE: &'static str = r#""OvertimeRequestId"=$1,"EmployeeId"=$2,"OvertimeDate"=$3,"OvertimeHours"=$4,"Reason"=$5,"RequestDate"=$6,"StatusId"=$7 WHERE "OvertimeRequestId"=$1"#;

    pub fn get_id(&self) -> Uuid {
        self.overtime_request_id.clone()
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
