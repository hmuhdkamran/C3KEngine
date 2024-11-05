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
pub struct Allowances {
    pub allowance_id: Uuid,
pub abbreviation: String,
pub full_name: String,
pub is_taxable: bool,
pub status_id: Uuid

}

impl Allowances {
    pub const TABLE: &'static str = r#""Payroll"."Allowances""#;
    pub const PK: &'static str = "AllowanceId";
    pub const COLUMNS_ARRAY: [&'static str; 5] = ["AllowanceId","Abbreviation","FullName","IsTaxable","StatusId"];

    pub fn get_id(&self) -> Uuid {
        self.allowance_id.clone()
    }

    pub fn get_args(&self) -> PgArguments {
        let mut args = PgArguments::default();
        let _ = args.add(self.allowance_id.clone());
let _ = args.add(self.abbreviation.clone());
let _ = args.add(self.full_name.clone());
let _ = args.add(self.is_taxable.clone());
let _ = args.add(self.status_id.clone());

        args
    }

    pub fn new(allowance_id: Uuid,abbreviation: String,full_name: String,is_taxable: bool,status_id: Uuid) -> Self {
        Self {
            allowance_id,
abbreviation,
full_name,
is_taxable,
status_id

        }
    }
}

impl PartialEq for Allowances {
    fn eq(&self, other: &Self) -> bool {
        self.allowance_id == other.allowance_id
    }
}

impl Model for Allowances {
    fn from_row(row: &PgRow) -> Allowances
    where
        Self: Sized,
    {
        let allowance_id = row.get("AllowanceId");
let abbreviation = row.get("Abbreviation");
let full_name = row.get("FullName");
let is_taxable = row.get("IsTaxable");
let status_id = row.get("StatusId");


        Self {
            allowance_id,
abbreviation,
full_name,
is_taxable,
status_id

        }
    }
}
