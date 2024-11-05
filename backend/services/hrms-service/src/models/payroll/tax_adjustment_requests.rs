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
pub struct TaxAdjustmentRequests {
    pub tax_adjustment_request_id: Uuid,
pub employee_id: Uuid,
pub adjustment_date: DateTime<Utc>,
pub attached_document: String,
pub adjustment_amount: f64,
pub status_id: Uuid

}

impl TaxAdjustmentRequests {
    pub const TABLE: &'static str = r#""Payroll"."TaxAdjustmentRequests""#;
    pub const PK: &'static str = "TaxAdjustmentRequestId";
    pub const COLUMNS_ARRAY: [&'static str; 6] = ["TaxAdjustmentRequestId","EmployeeId","AdjustmentDate","AttachedDocument","AdjustmentAmount","StatusId"];

    pub fn get_id(&self) -> Uuid {
        self.tax_adjustment_request_id.clone()
    }

    pub fn get_args(&self) -> PgArguments {
        let mut args = PgArguments::default();
        let _ = args.add(self.tax_adjustment_request_id.clone());
let _ = args.add(self.employee_id.clone());
let _ = args.add(self.adjustment_date.clone());
let _ = args.add(self.attached_document.clone());
let _ = args.add(self.adjustment_amount.clone());
let _ = args.add(self.status_id.clone());

        args
    }

    pub fn new(tax_adjustment_request_id: Uuid,employee_id: Uuid,adjustment_date: DateTime<Utc>,attached_document: String,adjustment_amount: f64,status_id: Uuid) -> Self {
        Self {
            tax_adjustment_request_id,
employee_id,
adjustment_date,
attached_document,
adjustment_amount,
status_id

        }
    }
}

impl PartialEq for TaxAdjustmentRequests {
    fn eq(&self, other: &Self) -> bool {
        self.tax_adjustment_request_id == other.tax_adjustment_request_id
    }
}

impl Model for TaxAdjustmentRequests {
    fn from_row(row: &PgRow) -> TaxAdjustmentRequests
    where
        Self: Sized,
    {
        let tax_adjustment_request_id = row.get("TaxAdjustmentRequestId");
let employee_id = row.get("EmployeeId");
let adjustment_date = row.get("AdjustmentDate");
let attached_document = row.get("AttachedDocument");
let adjustment_amount = row.get("AdjustmentAmount");
let status_id = row.get("StatusId");


        Self {
            tax_adjustment_request_id,
employee_id,
adjustment_date,
attached_document,
adjustment_amount,
status_id

        }
    }
}
