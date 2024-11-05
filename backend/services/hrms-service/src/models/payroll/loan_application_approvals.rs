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
pub struct LoanApplicationApprovals {
    pub loan_application_approval_id: Uuid,
pub loan_application_id: Uuid,
pub approved_by: Uuid,
pub approved_date: DateTime<Utc>,
pub apporved_amount: f64,
pub remkars: String,
pub status_id: Uuid

}

impl LoanApplicationApprovals {
    pub const TABLE: &'static str = r#""Payroll"."LoanApplicationApprovals""#;
    pub const PK: &'static str = "LoanApplicationApprovalId";
    pub const COLUMNS_ARRAY: [&'static str; 7] = ["LoanApplicationApprovalId","LoanApplicationId","ApprovedBy","ApprovedDate","ApporvedAmount","Remkars","StatusId"];

    pub fn get_id(&self) -> Uuid {
        self.loan_application_approval_id.clone()
    }

    pub fn get_args(&self) -> PgArguments {
        let mut args = PgArguments::default();
        let _ = args.add(self.loan_application_approval_id.clone());
let _ = args.add(self.loan_application_id.clone());
let _ = args.add(self.approved_by.clone());
let _ = args.add(self.approved_date.clone());
let _ = args.add(self.apporved_amount.clone());
let _ = args.add(self.remkars.clone());
let _ = args.add(self.status_id.clone());

        args
    }

    pub fn new(loan_application_approval_id: Uuid,loan_application_id: Uuid,approved_by: Uuid,approved_date: DateTime<Utc>,apporved_amount: f64,remkars: String,status_id: Uuid) -> Self {
        Self {
            loan_application_approval_id,
loan_application_id,
approved_by,
approved_date,
apporved_amount,
remkars,
status_id

        }
    }
}

impl PartialEq for LoanApplicationApprovals {
    fn eq(&self, other: &Self) -> bool {
        self.loan_application_approval_id == other.loan_application_approval_id
    }
}

impl Model for LoanApplicationApprovals {
    fn from_row(row: &PgRow) -> LoanApplicationApprovals
    where
        Self: Sized,
    {
        let loan_application_approval_id = row.get("LoanApplicationApprovalId");
let loan_application_id = row.get("LoanApplicationId");
let approved_by = row.get("ApprovedBy");
let approved_date = row.get("ApprovedDate");
let apporved_amount = row.get("ApporvedAmount");
let remkars = row.get("Remkars");
let status_id = row.get("StatusId");


        Self {
            loan_application_approval_id,
loan_application_id,
approved_by,
approved_date,
apporved_amount,
remkars,
status_id

        }
    }
}
