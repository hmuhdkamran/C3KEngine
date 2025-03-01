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
pub struct LoanInstallmentPlanDetails {
    pub loan_installment_plan_detail_id: Uuid,
pub loan_installment_id: Uuid,
pub installment_amount: f64,
pub markup_rate: f64,
pub markup_amount: f64,
pub total_installment: f64,
pub status_id: Uuid

}

impl LoanInstallmentPlanDetails {
    pub const TABLE: &'static str = r#""Payroll"."LoanInstallmentPlanDetails""#;
    pub const PK: &'static str = "LoanInstallmentPlanDetailId";
    pub const COLUMNS_ARRAY: [&'static str; 7] = ["LoanInstallmentPlanDetailId","LoanInstallmentId","InstallmentAmount","MarkupRate","MarkupAmount","TotalInstallment","StatusId"];

    pub fn get_id(&self) -> Uuid {
        self.loan_installment_plan_detail_id.clone()
    }

    pub fn get_args(&self) -> PgArguments {
        let mut args = PgArguments::default();
        let _ = args.add(self.loan_installment_plan_detail_id.clone());
let _ = args.add(self.loan_installment_id.clone());
let _ = args.add(self.installment_amount.clone());
let _ = args.add(self.markup_rate.clone());
let _ = args.add(self.markup_amount.clone());
let _ = args.add(self.total_installment.clone());
let _ = args.add(self.status_id.clone());

        args
    }

    pub fn new(loan_installment_plan_detail_id: Uuid,loan_installment_id: Uuid,installment_amount: f64,markup_rate: f64,markup_amount: f64,total_installment: f64,status_id: Uuid) -> Self {
        Self {
            loan_installment_plan_detail_id,
loan_installment_id,
installment_amount,
markup_rate,
markup_amount,
total_installment,
status_id

        }
    }
}

impl PartialEq for LoanInstallmentPlanDetails {
    fn eq(&self, other: &Self) -> bool {
        self.loan_installment_plan_detail_id == other.loan_installment_plan_detail_id
    }
}

impl Model for LoanInstallmentPlanDetails {
    fn from_row(row: &PgRow) -> LoanInstallmentPlanDetails
    where
        Self: Sized,
    {
        let loan_installment_plan_detail_id = row.get("LoanInstallmentPlanDetailId");
let loan_installment_id = row.get("LoanInstallmentId");
let installment_amount = row.get("InstallmentAmount");
let markup_rate = row.get("MarkupRate");
let markup_amount = row.get("MarkupAmount");
let total_installment = row.get("TotalInstallment");
let status_id = row.get("StatusId");


        Self {
            loan_installment_plan_detail_id,
loan_installment_id,
installment_amount,
markup_rate,
markup_amount,
total_installment,
status_id

        }
    }
}
