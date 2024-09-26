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
pub struct Currencies {
    pub currency_id: Uuid,
pub code: String,
pub symbol: String,
pub status_id: Uuid

}

impl Currencies {
    pub const TABLE: &'static str = r#""Setup"."Currencies""#;
    pub const PK: &'static str = r#"CurrencyId::TEXT=$1"#;
    pub const COLUMNS: &'static str = r#""CurrencyId","Code","Symbol","StatusId""#;
    pub const COLUMNS_UPDATE: &'static str = r#""CurrencyId"=$1,"Code"=$2,"Symbol"=$3,"StatusId"=$4 WHERE "CurrencyId"=$1"#;

    pub fn get_id(&self) -> Uuid {
        self.currency_id.clone()
    }

    pub fn new(currency_id: Uuid,code: String,symbol: String,status_id: Uuid) -> Self {
        Self {
            currency_id,
code,
symbol,
status_id

        }
    }
}

impl PartialEq for Currencies {
    fn eq(&self, other: &Self) -> bool {
        self.currency_id == other.currency_id
    }
}

impl Model for Currencies {
    fn from_row(row: &PgRow) -> Currencies
    where
        Self: Sized,
    {
        let currency_id = row.get("CurrencyId");
let code = row.get("Code");
let symbol = row.get("Symbol");
let status_id = row.get("StatusId");


        Self {
            currency_id,
code,
symbol,
status_id

        }
    }
}
