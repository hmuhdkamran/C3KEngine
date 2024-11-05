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
pub struct FinancialYears {
    pub financial_year_id: Uuid,
pub start_date: DateTime<Utc>,
pub end_date: DateTime<Utc>,
pub status_id: Uuid

}

impl FinancialYears {
    pub const TABLE: &'static str = r#""Setting"."FinancialYears""#;
    pub const PK: &'static str = "FinancialYearId";
    pub const COLUMNS_ARRAY: [&'static str; 4] = ["FinancialYearId","StartDate","EndDate","StatusId"];

    pub fn get_id(&self) -> Uuid {
        self.financial_year_id.clone()
    }

    pub fn get_args(&self) -> PgArguments {
        let mut args = PgArguments::default();
        let _ = args.add(self.financial_year_id.clone());
let _ = args.add(self.start_date.clone());
let _ = args.add(self.end_date.clone());
let _ = args.add(self.status_id.clone());

        args
    }

    pub fn new(financial_year_id: Uuid,start_date: DateTime<Utc>,end_date: DateTime<Utc>,status_id: Uuid) -> Self {
        Self {
            financial_year_id,
start_date,
end_date,
status_id

        }
    }
}

impl PartialEq for FinancialYears {
    fn eq(&self, other: &Self) -> bool {
        self.financial_year_id == other.financial_year_id
    }
}

impl Model for FinancialYears {
    fn from_row(row: &PgRow) -> FinancialYears
    where
        Self: Sized,
    {
        let financial_year_id = row.get("FinancialYearId");
let start_date = row.get("StartDate");
let end_date = row.get("EndDate");
let status_id = row.get("StatusId");


        Self {
            financial_year_id,
start_date,
end_date,
status_id

        }
    }
}
