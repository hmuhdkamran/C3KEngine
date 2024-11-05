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
    pub const PK: &'static str = "IncomeTaxSlabId";
    pub const COLUMNS_ARRAY: [&'static str; 6] = ["IncomeTaxSlabId","StartAmount","EndAmount","ExceedAmount","TaxRate","StatusId"];

    pub fn get_id(&self) -> Uuid {
        self.income_tax_slab_id.clone()
    }

    pub fn get_args(&self) -> PgArguments {
        let mut args = PgArguments::default();
        let _ = args.add(self.income_tax_slab_id.clone());
let _ = args.add(self.start_amount.clone());
let _ = args.add(self.end_amount.clone());
let _ = args.add(self.exceed_amount.clone());
let _ = args.add(self.tax_rate.clone());
let _ = args.add(self.status_id.clone());

        args
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
