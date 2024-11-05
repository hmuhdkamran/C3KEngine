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
pub struct SalaryDeductions {
    pub salary_deduction_id: Uuid,
pub salary_id: Uuid,
pub deduction_id: Uuid,
pub deduction_amount: f64,
pub deduction_paid_amount: f64,
pub status_id: Uuid

}

impl SalaryDeductions {
    pub const TABLE: &'static str = r#""Payroll"."SalaryDeductions""#;
    pub const PK: &'static str = "SalaryDeductionId";
    pub const COLUMNS_ARRAY: [&'static str; 6] = ["SalaryDeductionId","SalaryId","DeductionId","DeductionAmount","DeductionPaidAmount","StatusId"];

    pub fn get_id(&self) -> Uuid {
        self.salary_deduction_id.clone()
    }

    pub fn get_args(&self) -> PgArguments {
        let mut args = PgArguments::default();
        let _ = args.add(self.salary_deduction_id.clone());
let _ = args.add(self.salary_id.clone());
let _ = args.add(self.deduction_id.clone());
let _ = args.add(self.deduction_amount.clone());
let _ = args.add(self.deduction_paid_amount.clone());
let _ = args.add(self.status_id.clone());

        args
    }

    pub fn new(salary_deduction_id: Uuid,salary_id: Uuid,deduction_id: Uuid,deduction_amount: f64,deduction_paid_amount: f64,status_id: Uuid) -> Self {
        Self {
            salary_deduction_id,
salary_id,
deduction_id,
deduction_amount,
deduction_paid_amount,
status_id

        }
    }
}

impl PartialEq for SalaryDeductions {
    fn eq(&self, other: &Self) -> bool {
        self.salary_deduction_id == other.salary_deduction_id
    }
}

impl Model for SalaryDeductions {
    fn from_row(row: &PgRow) -> SalaryDeductions
    where
        Self: Sized,
    {
        let salary_deduction_id = row.get("SalaryDeductionId");
let salary_id = row.get("SalaryId");
let deduction_id = row.get("DeductionId");
let deduction_amount = row.get("DeductionAmount");
let deduction_paid_amount = row.get("DeductionPaidAmount");
let status_id = row.get("StatusId");


        Self {
            salary_deduction_id,
salary_id,
deduction_id,
deduction_amount,
deduction_paid_amount,
status_id

        }
    }
}
