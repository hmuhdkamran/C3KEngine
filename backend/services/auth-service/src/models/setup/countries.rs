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
pub struct Countries {
    pub country_id: Uuid,
pub abbreviation: String,
pub full_name: String,
pub currency: String,
pub status_id: Uuid

}

impl Countries {
    pub const TABLE: &'static str = r#""Setup"."Countries""#;
    pub const PK: &'static str = "CountryId";
    pub const COLUMNS_ARRAY: [&'static str; 5] = ["CountryId","Abbreviation","FullName","Currency","StatusId"];

    pub fn get_id(&self) -> Uuid {
        self.country_id.clone()
    }

    pub fn get_args(&self) -> PgArguments {
        let mut args = PgArguments::default();
        let _ = args.add(self.country_id.clone());
let _ = args.add(self.abbreviation.clone());
let _ = args.add(self.full_name.clone());
let _ = args.add(self.currency.clone());
let _ = args.add(self.status_id.clone());

        args
    }

    pub fn new(country_id: Uuid,abbreviation: String,full_name: String,currency: String,status_id: Uuid) -> Self {
        Self {
            country_id,
abbreviation,
full_name,
currency,
status_id

        }
    }
}

impl PartialEq for Countries {
    fn eq(&self, other: &Self) -> bool {
        self.country_id == other.country_id
    }
}

impl Model for Countries {
    fn from_row(row: &PgRow) -> Countries
    where
        Self: Sized,
    {
        let country_id = row.get("CountryId");
let abbreviation = row.get("Abbreviation");
let full_name = row.get("FullName");
let currency = row.get("Currency");
let status_id = row.get("StatusId");


        Self {
            country_id,
abbreviation,
full_name,
currency,
status_id

        }
    }
}
