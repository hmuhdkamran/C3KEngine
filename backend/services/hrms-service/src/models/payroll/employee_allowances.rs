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
pub struct EmployeeAllowances {
    pub employee_allowance_id: Uuid,
pub allowance_id: Uuid,
pub employee_id: Uuid,
pub amount: f64,
pub status_id: Uuid

}

impl EmployeeAllowances {
    pub const TABLE: &'static str = r#""Payroll"."EmployeeAllowances""#;
    pub const PK: &'static str = r#"EmployeeAllowanceId::TEXT=$1"#;
    pub const COLUMNS: &'static str = r#""EmployeeAllowanceId","AllowanceId","EmployeeId","Amount","StatusId""#;
    pub const COLUMNS_UPDATE: &'static str = r#""EmployeeAllowanceId"=$1,"AllowanceId"=$2,"EmployeeId"=$3,"Amount"=$4,"StatusId"=$5 WHERE "EmployeeAllowanceId"=$1"#;

    pub fn get_id(&self) -> Uuid {
        self.employee_allowance_id.clone()
    }

    pub fn new(employee_allowance_id: Uuid,allowance_id: Uuid,employee_id: Uuid,amount: f64,status_id: Uuid) -> Self {
        Self {
            employee_allowance_id,
allowance_id,
employee_id,
amount,
status_id

        }
    }
}

impl PartialEq for EmployeeAllowances {
    fn eq(&self, other: &Self) -> bool {
        self.employee_allowance_id == other.employee_allowance_id
    }
}

impl Model for EmployeeAllowances {
    fn from_row(row: &PgRow) -> EmployeeAllowances
    where
        Self: Sized,
    {
        let employee_allowance_id = row.get("EmployeeAllowanceId");
let allowance_id = row.get("AllowanceId");
let employee_id = row.get("EmployeeId");
let amount = row.get("Amount");
let status_id = row.get("StatusId");


        Self {
            employee_allowance_id,
allowance_id,
employee_id,
amount,
status_id

        }
    }
}
