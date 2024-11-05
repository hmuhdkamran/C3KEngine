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
pub struct Businesses {
    pub business_id: Uuid,
pub full_name: String,
pub abbreviation: String,
pub status_id: Uuid

}

impl Businesses {
    pub const TABLE: &'static str = r#""Setup"."Businesses""#;
    pub const PK: &'static str = "BusinessId";
    pub const COLUMNS_ARRAY: [&'static str; 4] = ["BusinessId","FullName","Abbreviation","StatusId"];

    pub fn get_id(&self) -> Uuid {
        self.business_id.clone()
    }

    pub fn get_args(&self) -> PgArguments {
        let mut args = PgArguments::default();
        let _ = args.add(self.business_id.clone());
let _ = args.add(self.full_name.clone());
let _ = args.add(self.abbreviation.clone());
let _ = args.add(self.status_id.clone());

        args
    }

    pub fn new(business_id: Uuid,full_name: String,abbreviation: String,status_id: Uuid) -> Self {
        Self {
            business_id,
full_name,
abbreviation,
status_id

        }
    }
}

impl PartialEq for Businesses {
    fn eq(&self, other: &Self) -> bool {
        self.business_id == other.business_id
    }
}

impl Model for Businesses {
    fn from_row(row: &PgRow) -> Businesses
    where
        Self: Sized,
    {
        let business_id = row.get("BusinessId");
let full_name = row.get("FullName");
let abbreviation = row.get("Abbreviation");
let status_id = row.get("StatusId");


        Self {
            business_id,
full_name,
abbreviation,
status_id

        }
    }
}
