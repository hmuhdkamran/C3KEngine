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
pub struct LoanInstallmentPlans {
    pub loan_installment_plan_id: Uuid,
pub plan_date: DateTime<Utc>,
pub loan_application_approval_id: Uuid,
pub loan_markup_rate_id: Uuid,
pub markup_rate: f64,
pub status_id: Uuid,
pub loan_amount: f64,
pub no_of_installment: f64

}

impl LoanInstallmentPlans {
    pub const TABLE: &'static str = r#""Payroll"."LoanInstallmentPlans""#;
    pub const PK: &'static str = "LoanInstallmentPlanId";
    pub const COLUMNS_ARRAY: [&'static str; 8] = ["LoanInstallmentPlanId","PlanDate","LoanApplicationApprovalId","LoanMarkupRateId","MarkupRate","StatusId","LoanAmount","NoOfInstallment"];

    pub fn get_id(&self) -> Uuid {
        self.loan_installment_plan_id.clone()
    }

    pub fn get_args(&self) -> PgArguments {
        let mut args = PgArguments::default();
        let _ = args.add(self.loan_installment_plan_id.clone());
let _ = args.add(self.plan_date.clone());
let _ = args.add(self.loan_application_approval_id.clone());
let _ = args.add(self.loan_markup_rate_id.clone());
let _ = args.add(self.markup_rate.clone());
let _ = args.add(self.status_id.clone());
let _ = args.add(self.loan_amount.clone());
let _ = args.add(self.no_of_installment.clone());

        args
    }

    pub fn new(loan_installment_plan_id: Uuid,plan_date: DateTime<Utc>,loan_application_approval_id: Uuid,loan_markup_rate_id: Uuid,markup_rate: f64,status_id: Uuid,loan_amount: f64,no_of_installment: f64) -> Self {
        Self {
            loan_installment_plan_id,
plan_date,
loan_application_approval_id,
loan_markup_rate_id,
markup_rate,
status_id,
loan_amount,
no_of_installment

        }
    }
}

impl PartialEq for LoanInstallmentPlans {
    fn eq(&self, other: &Self) -> bool {
        self.loan_installment_plan_id == other.loan_installment_plan_id
    }
}

impl Model for LoanInstallmentPlans {
    fn from_row(row: &PgRow) -> LoanInstallmentPlans
    where
        Self: Sized,
    {
        let loan_installment_plan_id = row.get("LoanInstallmentPlanId");
let plan_date = row.get("PlanDate");
let loan_application_approval_id = row.get("LoanApplicationApprovalId");
let loan_markup_rate_id = row.get("LoanMarkupRateId");
let markup_rate = row.get("MarkupRate");
let status_id = row.get("StatusId");
let loan_amount = row.get("LoanAmount");
let no_of_installment = row.get("NoOfInstallment");


        Self {
            loan_installment_plan_id,
plan_date,
loan_application_approval_id,
loan_markup_rate_id,
markup_rate,
status_id,
loan_amount,
no_of_installment

        }
    }
}
