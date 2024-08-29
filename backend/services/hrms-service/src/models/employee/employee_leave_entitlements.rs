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
pub struct EmployeeLeaveEntitlements {
    pub employee_leave_entitlement_id: Uuid,
pub employee_id: Uuid,
pub leave_type_id: Uuid,
pub entitlement: f64,
pub availed: f64,
pub balanced: f64,
pub status_id: Uuid

}

impl EmployeeLeaveEntitlements {
    pub const TABLE: &'static str = r#""Employee"."EmployeeLeaveEntitlements""#;
    pub const PK: &'static str = r#"EmployeeLeaveEntitlementId::TEXT=$1"#;
    pub const COLUMNS: &'static str = r#""EmployeeLeaveEntitlementId","EmployeeId","LeaveTypeId","Entitlement","Availed","Balanced","StatusId""#;
    pub const COLUMNS_UPDATE: &'static str = r#""EmployeeLeaveEntitlementId"=$1,"EmployeeId"=$2,"LeaveTypeId"=$3,"Entitlement"=$4,"Availed"=$5,"Balanced"=$6,"StatusId"=$7 WHERE "EmployeeLeaveEntitlementId"=$1"#;

    pub fn get_id(&self) -> Uuid {
        self.employee_leave_entitlement_id.clone()
    }

    pub fn new(employee_leave_entitlement_id: Uuid,employee_id: Uuid,leave_type_id: Uuid,entitlement: f64,availed: f64,balanced: f64,status_id: Uuid) -> Self {
        Self {
            employee_leave_entitlement_id,
employee_id,
leave_type_id,
entitlement,
availed,
balanced,
status_id

        }
    }
}

impl PartialEq for EmployeeLeaveEntitlements {
    fn eq(&self, other: &Self) -> bool {
        self.employee_leave_entitlement_id == other.employee_leave_entitlement_id
    }
}

impl Model for EmployeeLeaveEntitlements {
    fn from_row(row: &PgRow) -> EmployeeLeaveEntitlements
    where
        Self: Sized,
    {
        let employee_leave_entitlement_id = row.get("EmployeeLeaveEntitlementId");
let employee_id = row.get("EmployeeId");
let leave_type_id = row.get("LeaveTypeId");
let entitlement = row.get("Entitlement");
let availed = row.get("Availed");
let balanced = row.get("Balanced");
let status_id = row.get("StatusId");


        Self {
            employee_leave_entitlement_id,
employee_id,
leave_type_id,
entitlement,
availed,
balanced,
status_id

        }
    }
}
