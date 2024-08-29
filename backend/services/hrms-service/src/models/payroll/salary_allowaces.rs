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
pub struct SalaryAllowaces {
    pub salary_allowace_id: Uuid,
pub allowance_id: Uuid,
pub allowance_amount: f64,
pub allowance_paid_amount: f64,
pub status_id: Uuid,
pub salary_id: Uuid

}

impl SalaryAllowaces {
    pub const TABLE: &'static str = r#""Payroll"."SalaryAllowaces""#;
    pub const PK: &'static str = r#"SalaryAllowaceId::TEXT=$1"#;
    pub const COLUMNS: &'static str = r#""SalaryAllowaceId","AllowanceId","AllowanceAmount","AllowancePaidAmount","StatusId","SalaryId""#;
    pub const COLUMNS_UPDATE: &'static str = r#""SalaryAllowaceId"=$1,"AllowanceId"=$2,"AllowanceAmount"=$3,"AllowancePaidAmount"=$4,"StatusId"=$5,"SalaryId"=$6 WHERE "SalaryAllowaceId"=$1"#;

    pub fn get_id(&self) -> Uuid {
        self.salary_allowace_id.clone()
    }

    pub fn new(salary_allowace_id: Uuid,allowance_id: Uuid,allowance_amount: f64,allowance_paid_amount: f64,status_id: Uuid,salary_id: Uuid) -> Self {
        Self {
            salary_allowace_id,
allowance_id,
allowance_amount,
allowance_paid_amount,
status_id,
salary_id

        }
    }
}

impl PartialEq for SalaryAllowaces {
    fn eq(&self, other: &Self) -> bool {
        self.salary_allowace_id == other.salary_allowace_id
    }
}

impl Model for SalaryAllowaces {
    fn from_row(row: &PgRow) -> SalaryAllowaces
    where
        Self: Sized,
    {
        let salary_allowace_id = row.get("SalaryAllowaceId");
let allowance_id = row.get("AllowanceId");
let allowance_amount = row.get("AllowanceAmount");
let allowance_paid_amount = row.get("AllowancePaidAmount");
let status_id = row.get("StatusId");
let salary_id = row.get("SalaryId");


        Self {
            salary_allowace_id,
allowance_id,
allowance_amount,
allowance_paid_amount,
status_id,
salary_id

        }
    }
}
