use c3k_common::interfaces::irepository::Model;
use serde::{Deserialize, Serialize};
pub use sqlx::{
    pool::PoolConnection,
    postgres::{PgArguments, PgPoolOptions, PgRow},
    types::chrono::{DateTime, Utc},
    Arguments, Error, PgPool, Postgres, Row,
};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaxRates {
    pub tax_rate_id: Uuid,
    pub full_name: String,
    pub tax_rate: f64,
    pub status_id: Uuid,
}

impl TaxRates {
    pub const TABLE: &'static str = r#""Setup"."TaxRates""#;
    pub const PK: &'static str = r#"TaxRateId::TEXT=$1"#;
    pub const COLUMNS: &'static str = r#""TaxRateId","TaxRateId","FullName","TaxRate","StatusId""#;
    pub const COLUMNS_UPDATE: &'static str = r#""TaxRateId"=$1,"FullName"=$2,"TaxRate"=$3,"StatusId"=$4 WHERE "TaxRateId"=$1"#;

    pub fn get_id(&self) -> Uuid {
        self.tax_rate_id.clone()
    }

    pub fn new(tax_rate_id: Uuid, full_name: String, tax_rate: f64, status_id: Uuid) -> Self {
        Self {
            tax_rate_id,
            full_name,
            tax_rate,
            status_id,
        }
    }
}

impl PartialEq for TaxRates {
    fn eq(&self, other: &Self) -> bool {
        self.tax_rate_id == other.tax_rate_id
    }
}

impl Model for TaxRates {
    fn from_row(row: &PgRow) -> TaxRates
    where
        Self: Sized,
    {
        let tax_rate_id = row.get("TaxRateId");
        let full_name = row.get("FullName");
        let tax_rate = row.get("TaxRate");
        let status_id = row.get("StatusId");

        Self {
            tax_rate_id,
            full_name,
            tax_rate,
            status_id,
        }
    }
}
