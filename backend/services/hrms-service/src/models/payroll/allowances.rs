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
pub struct Allowances {
    pub allowance_id: Uuid,
pub abbreviation: String,
pub full_name: String,
pub is_taxable: bool,
pub status_id: Uuid

}

impl Allowances {
    pub const TABLE: &'static str = r#""Payroll"."Allowances""#;
    pub const PK: &'static str = r#"AllowanceId::TEXT=$1"#;
    pub const COLUMNS: &'static str = r#""AllowanceId","Abbreviation","FullName","IsTaxable","StatusId""#;
    pub const COLUMNS_UPDATE: &'static str = r#""AllowanceId"=$1,"Abbreviation"=$2,"FullName"=$3,"IsTaxable"=$4,"StatusId"=$5 WHERE "AllowanceId"=$1"#;

    pub fn get_id(&self) -> Uuid {
        self.allowance_id.clone()
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
