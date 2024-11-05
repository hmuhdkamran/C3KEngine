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
pub struct EmployeeDeductions {
    pub employee_deduction_id: Uuid,
pub employee_id: Uuid,
pub deduction_id: Uuid,
pub deduction_amount: f64,
pub status_id: Uuid

}

impl EmployeeDeductions {
    pub const TABLE: &'static str = r#""Payroll"."EmployeeDeductions""#;
    pub const PK: &'static str = "EmployeeDeductionId";
    pub const COLUMNS_ARRAY: [&'static str; 5] = ["EmployeeDeductionId","EmployeeId","DeductionId","DeductionAmount","StatusId"];

    pub fn get_id(&self) -> Uuid {
        self.employee_deduction_id.clone()
    }

    pub fn get_args(&self) -> PgArguments {
        let mut args = PgArguments::default();
        let _ = args.add(self.employee_deduction_id.clone());
let _ = args.add(self.employee_id.clone());
let _ = args.add(self.deduction_id.clone());
let _ = args.add(self.deduction_amount.clone());
let _ = args.add(self.status_id.clone());

        args
    }

    pub fn new(employee_deduction_id: Uuid,employee_id: Uuid,deduction_id: Uuid,deduction_amount: f64,status_id: Uuid) -> Self {
        Self {
            employee_deduction_id,
employee_id,
deduction_id,
deduction_amount,
status_id

        }
    }
}

impl PartialEq for EmployeeDeductions {
    fn eq(&self, other: &Self) -> bool {
        self.employee_deduction_id == other.employee_deduction_id
    }
}

impl Model for EmployeeDeductions {
    fn from_row(row: &PgRow) -> EmployeeDeductions
    where
        Self: Sized,
    {
        let employee_deduction_id = row.get("EmployeeDeductionId");
let employee_id = row.get("EmployeeId");
let deduction_id = row.get("DeductionId");
let deduction_amount = row.get("DeductionAmount");
let status_id = row.get("StatusId");


        Self {
            employee_deduction_id,
employee_id,
deduction_id,
deduction_amount,
status_id

        }
    }
}
