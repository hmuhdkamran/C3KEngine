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
pub struct Currencies {
    pub currency_id: Uuid,
pub code: String,
pub symbol: String,
pub status_id: Uuid

}

impl Currencies {
    pub const TABLE: &'static str = r#""Setup"."Currencies""#;
    pub const PK: &'static str = "CurrencyId";
    pub const COLUMNS_ARRAY: [&'static str; 4] = ["CurrencyId","Code","Symbol","StatusId"];

    pub fn get_id(&self) -> Uuid {
        self.currency_id.clone()
    }

    pub fn get_args(&self) -> PgArguments {
        let mut args = PgArguments::default();
        let _ = args.add(self.currency_id.clone());
let _ = args.add(self.code.clone());
let _ = args.add(self.symbol.clone());
let _ = args.add(self.status_id.clone());

        args
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
