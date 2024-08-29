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
pub struct LoanMarkupRates {
    pub loan_markup_rate_id: Uuid,
pub markup_rate: f64,
pub financial_year_id: Uuid,
pub status_id: Uuid

}

impl LoanMarkupRates {
    pub const TABLE: &'static str = r#""Payroll"."LoanMarkupRates""#;
    pub const PK: &'static str = r#"LoanMarkupRateId::TEXT=$1"#;
    pub const COLUMNS: &'static str = r#""LoanMarkupRateId","MarkupRate","FinancialYearId","StatusId""#;
    pub const COLUMNS_UPDATE: &'static str = r#""LoanMarkupRateId"=$1,"MarkupRate"=$2,"FinancialYearId"=$3,"StatusId"=$4 WHERE "LoanMarkupRateId"=$1"#;

    pub fn get_id(&self) -> Uuid {
        self.loan_markup_rate_id.clone()
    }

    pub fn new(loan_markup_rate_id: Uuid,markup_rate: f64,financial_year_id: Uuid,status_id: Uuid) -> Self {
        Self {
            loan_markup_rate_id,
markup_rate,
financial_year_id,
status_id

        }
    }
}

impl PartialEq for LoanMarkupRates {
    fn eq(&self, other: &Self) -> bool {
        self.loan_markup_rate_id == other.loan_markup_rate_id
    }
}

impl Model for LoanMarkupRates {
    fn from_row(row: &PgRow) -> LoanMarkupRates
    where
        Self: Sized,
    {
        let loan_markup_rate_id = row.get("LoanMarkupRateId");
let markup_rate = row.get("MarkupRate");
let financial_year_id = row.get("FinancialYearId");
let status_id = row.get("StatusId");


        Self {
            loan_markup_rate_id,
markup_rate,
financial_year_id,
status_id

        }
    }
}
