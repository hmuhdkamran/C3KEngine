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
pub struct EmployeeShifts {
    pub employee_shift_id: Uuid,
pub employee_id: Uuid,
pub shift_id: Uuid,
pub approved_by: Uuid,
pub application_status_id: Uuid,
pub status_id: Uuid

}

impl EmployeeShifts {
    pub const TABLE: &'static str = r#""Attendance"."EmployeeShifts""#;
    pub const PK: &'static str = r#"EmployeeShiftId::TEXT=$1"#;
    pub const COLUMNS: &'static str = r#""EmployeeShiftId","EmployeeId","ShiftId","ApprovedBy","ApplicationStatusId","StatusId""#;
    pub const COLUMNS_UPDATE: &'static str = r#""EmployeeShiftId"=$1,"EmployeeId"=$2,"ShiftId"=$3,"ApprovedBy"=$4,"ApplicationStatusId"=$5,"StatusId"=$6 WHERE "EmployeeShiftId"=$1"#;

    pub fn get_id(&self) -> Uuid {
        self.employee_shift_id.clone()
    }

    pub fn new(employee_shift_id: Uuid,employee_id: Uuid,shift_id: Uuid,approved_by: Uuid,application_status_id: Uuid,status_id: Uuid) -> Self {
        Self {
            employee_shift_id,
employee_id,
shift_id,
approved_by,
application_status_id,
status_id

        }
    }
}

impl PartialEq for EmployeeShifts {
    fn eq(&self, other: &Self) -> bool {
        self.employee_shift_id == other.employee_shift_id
    }
}

impl Model for EmployeeShifts {
    fn from_row(row: &PgRow) -> EmployeeShifts
    where
        Self: Sized,
    {
        let employee_shift_id = row.get("EmployeeShiftId");
let employee_id = row.get("EmployeeId");
let shift_id = row.get("ShiftId");
let approved_by = row.get("ApprovedBy");
let application_status_id = row.get("ApplicationStatusId");
let status_id = row.get("StatusId");


        Self {
            employee_shift_id,
employee_id,
shift_id,
approved_by,
application_status_id,
status_id

        }
    }
}
