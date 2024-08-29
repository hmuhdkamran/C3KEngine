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
    pub const PK: &'static str = r#"TaxAdjustmentApprovalId::TEXT=$1"#;
    pub const COLUMNS: &'static str = r#""TaxAdjustmentApprovalId","TaxAdjustmentId","ApprovedDate","Comments","ApprovedAmount","StatusId","ApprovedBy""#;
    pub const COLUMNS_UPDATE: &'static str = r#""TaxAdjustmentApprovalId"=$1,"TaxAdjustmentId"=$2,"ApprovedDate"=$3,"Comments"=$4,"ApprovedAmount"=$5,"StatusId"=$6,"ApprovedBy"=$7 WHERE "TaxAdjustmentApprovalId"=$1"#;

    pub fn get_id(&self) -> Uuid {
        self.tax_adjustment_approval_id.clone()
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
