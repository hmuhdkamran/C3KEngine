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
pub struct IncomeTaxSlabs {
    pub income_tax_slab_id: Uuid,
pub start_amount: f64,
pub end_amount: f64,
pub exceed_amount: f64,
pub tax_rate: f64,
pub status_id: Uuid

}

impl IncomeTaxSlabs {
    pub const TABLE: &'static str = r#""Payroll"."IncomeTaxSlabs""#;
    pub const PK: &'static str = r#"IncomeTaxSlabId::TEXT=$1"#;
    pub const COLUMNS: &'static str = r#""IncomeTaxSlabId","StartAmount","EndAmount","ExceedAmount","TaxRate","StatusId""#;
    pub const COLUMNS_UPDATE: &'static str = r#""IncomeTaxSlabId"=$1,"StartAmount"=$2,"EndAmount"=$3,"ExceedAmount"=$4,"TaxRate"=$5,"StatusId"=$6 WHERE "IncomeTaxSlabId"=$1"#;

    pub fn get_id(&self) -> Uuid {
        self.income_tax_slab_id.clone()
    }

    pub fn new(income_tax_slab_id: Uuid,start_amount: f64,end_amount: f64,exceed_amount: f64,tax_rate: f64,status_id: Uuid) -> Self {
        Self {
            income_tax_slab_id,
start_amount,
end_amount,
exceed_amount,
tax_rate,
status_id

        }
    }
}

impl PartialEq for IncomeTaxSlabs {
    fn eq(&self, other: &Self) -> bool {
        self.income_tax_slab_id == other.income_tax_slab_id
    }
}

impl Model for IncomeTaxSlabs {
    fn from_row(row: &PgRow) -> IncomeTaxSlabs
    where
        Self: Sized,
    {
        let income_tax_slab_id = row.get("IncomeTaxSlabId");
let start_amount = row.get("StartAmount");
let end_amount = row.get("EndAmount");
let exceed_amount = row.get("ExceedAmount");
let tax_rate = row.get("TaxRate");
let status_id = row.get("StatusId");


        Self {
            income_tax_slab_id,
start_amount,
end_amount,
exceed_amount,
tax_rate,
status_id

        }
    }
}
