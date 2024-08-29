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
pub struct Countries {
    pub country_id: Uuid,
pub code: String,
pub abbrevation: String,
pub name: String,
pub status_id: Uuid

}

impl Countries {
    pub const TABLE: &'static str = r#""Setting"."Countries""#;
    pub const PK: &'static str = r#"CountryId::TEXT=$1"#;
    pub const COLUMNS: &'static str = r#""CountryId","Code","Abbrevation","Name","StatusId""#;
    pub const COLUMNS_UPDATE: &'static str = r#""CountryId"=$1,"Code"=$2,"Abbrevation"=$3,"Name"=$4,"StatusId"=$5 WHERE "CountryId"=$1"#;

    pub fn get_id(&self) -> Uuid {
        self.country_id.clone()
    }

    pub fn new(country_id: Uuid,code: String,abbrevation: String,name: String,status_id: Uuid) -> Self {
        Self {
            country_id,
code,
abbrevation,
name,
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
let code = row.get("Code");
let abbrevation = row.get("Abbrevation");
let name = row.get("Name");
let status_id = row.get("StatusId");


        Self {
            country_id,
code,
abbrevation,
name,
status_id

        }
    }
}
