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
pub struct LoanApplications {
    pub loan_application_id: Uuid,
pub application_date: DateTime<Utc>,
pub employee_id: Uuid,
pub loan_type_id: Uuid,
pub loan_amount: f64,
pub application_status_id: Uuid,
pub remarks: String,
pub status_id: Uuid,
pub number_of_installment: f64

}

impl LoanApplications {
    pub const TABLE: &'static str = r#""Payroll"."LoanApplications""#;
    pub const PK: &'static str = "LoanApplicationId";
    pub const COLUMNS_ARRAY: [&'static str; 9] = ["LoanApplicationId","ApplicationDate","EmployeeId","LoanTypeId","LoanAmount","ApplicationStatusId","Remarks","StatusId","NumberOfInstallment"];

    pub fn get_id(&self) -> Uuid {
        self.loan_application_id.clone()
    }

    pub fn get_args(&self) -> PgArguments {
        let mut args = PgArguments::default();
        let _ = args.add(self.loan_application_id.clone());
let _ = args.add(self.application_date.clone());
let _ = args.add(self.employee_id.clone());
let _ = args.add(self.loan_type_id.clone());
let _ = args.add(self.loan_amount.clone());
let _ = args.add(self.application_status_id.clone());
let _ = args.add(self.remarks.clone());
let _ = args.add(self.status_id.clone());
let _ = args.add(self.number_of_installment.clone());

        args
    }

    pub fn new(loan_application_id: Uuid,application_date: DateTime<Utc>,employee_id: Uuid,loan_type_id: Uuid,loan_amount: f64,application_status_id: Uuid,remarks: String,status_id: Uuid,number_of_installment: f64) -> Self {
        Self {
            loan_application_id,
application_date,
employee_id,
loan_type_id,
loan_amount,
application_status_id,
remarks,
status_id,
number_of_installment

        }
    }
}

impl PartialEq for LoanApplications {
    fn eq(&self, other: &Self) -> bool {
        self.loan_application_id == other.loan_application_id
    }
}

impl Model for LoanApplications {
    fn from_row(row: &PgRow) -> LoanApplications
    where
        Self: Sized,
    {
        let loan_application_id = row.get("LoanApplicationId");
let application_date = row.get("ApplicationDate");
let employee_id = row.get("EmployeeId");
let loan_type_id = row.get("LoanTypeId");
let loan_amount = row.get("LoanAmount");
let application_status_id = row.get("ApplicationStatusId");
let remarks = row.get("Remarks");
let status_id = row.get("StatusId");
let number_of_installment = row.get("NumberOfInstallment");


        Self {
            loan_application_id,
application_date,
employee_id,
loan_type_id,
loan_amount,
application_status_id,
remarks,
status_id,
number_of_installment

        }
    }
}
