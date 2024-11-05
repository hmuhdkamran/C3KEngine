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
pub struct TaxAdjustmentApprovals {
    pub tax_adjustment_approval_id: Uuid,
pub tax_adjustment_id: Uuid,
pub approved_date: DateTime<Utc>,
pub comments: String,
pub approved_amount: f64,
pub status_id: Uuid,
pub approved_by: Uuid

}

impl TaxAdjustmentApprovals {
    pub const TABLE: &'static str = r#""Payroll"."TaxAdjustmentApprovals""#;
    pub const PK: &'static str = "TaxAdjustmentApprovalId";
    pub const COLUMNS_ARRAY: [&'static str; 7] = ["TaxAdjustmentApprovalId","TaxAdjustmentId","ApprovedDate","Comments","ApprovedAmount","StatusId","ApprovedBy"];

    pub fn get_id(&self) -> Uuid {
        self.tax_adjustment_approval_id.clone()
    }

    pub fn get_args(&self) -> PgArguments {
        let mut args = PgArguments::default();
        let _ = args.add(self.tax_adjustment_approval_id.clone());
let _ = args.add(self.tax_adjustment_id.clone());
let _ = args.add(self.approved_date.clone());
let _ = args.add(self.comments.clone());
let _ = args.add(self.approved_amount.clone());
let _ = args.add(self.status_id.clone());
let _ = args.add(self.approved_by.clone());

        args
    }

    pub fn new(tax_adjustment_approval_id: Uuid,tax_adjustment_id: Uuid,approved_date: DateTime<Utc>,comments: String,approved_amount: f64,status_id: Uuid,approved_by: Uuid) -> Self {
        Self {
            tax_adjustment_approval_id,
tax_adjustment_id,
approved_date,
comments,
approved_amount,
status_id,
approved_by

        }
    }
}

impl PartialEq for TaxAdjustmentApprovals {
    fn eq(&self, other: &Self) -> bool {
        self.tax_adjustment_approval_id == other.tax_adjustment_approval_id
    }
}

impl Model for TaxAdjustmentApprovals {
    fn from_row(row: &PgRow) -> TaxAdjustmentApprovals
    where
        Self: Sized,
    {
        let tax_adjustment_approval_id = row.get("TaxAdjustmentApprovalId");
let tax_adjustment_id = row.get("TaxAdjustmentId");
let approved_date = row.get("ApprovedDate");
let comments = row.get("Comments");
let approved_amount = row.get("ApprovedAmount");
let status_id = row.get("StatusId");
let approved_by = row.get("ApprovedBy");


        Self {
            tax_adjustment_approval_id,
tax_adjustment_id,
approved_date,
comments,
approved_amount,
status_id,
approved_by

        }
    }
}
