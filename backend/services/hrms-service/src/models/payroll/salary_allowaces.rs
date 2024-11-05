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
    pub const PK: &'static str = "SalaryAllowaceId";
    pub const COLUMNS_ARRAY: [&'static str; 6] = ["SalaryAllowaceId","AllowanceId","AllowanceAmount","AllowancePaidAmount","StatusId","SalaryId"];

    pub fn get_id(&self) -> Uuid {
        self.salary_allowace_id.clone()
    }

    pub fn get_args(&self) -> PgArguments {
        let mut args = PgArguments::default();
        let _ = args.add(self.salary_allowace_id.clone());
let _ = args.add(self.allowance_id.clone());
let _ = args.add(self.allowance_amount.clone());
let _ = args.add(self.allowance_paid_amount.clone());
let _ = args.add(self.status_id.clone());
let _ = args.add(self.salary_id.clone());

        args
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
