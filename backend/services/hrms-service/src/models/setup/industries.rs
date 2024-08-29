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
pub struct Industries {
    pub industry_id: Uuid,
pub abbreviation: String,
pub full_name: String,
pub status_id: Uuid

}

impl Industries {
    pub const TABLE: &'static str = r#""Setup"."Industries""#;
    pub const PK: &'static str = r#"IndustryId::TEXT=$1"#;
    pub const COLUMNS: &'static str = r#""IndustryId","Abbreviation","FullName","StatusId""#;
    pub const COLUMNS_UPDATE: &'static str = r#""IndustryId"=$1,"Abbreviation"=$2,"FullName"=$3,"StatusId"=$4 WHERE "IndustryId"=$1"#;

    pub fn get_id(&self) -> Uuid {
        self.industry_id.clone()
    }

    pub fn new(industry_id: Uuid,abbreviation: String,full_name: String,status_id: Uuid) -> Self {
        Self {
            industry_id,
abbreviation,
full_name,
status_id

        }
    }
}

impl PartialEq for Industries {
    fn eq(&self, other: &Self) -> bool {
        self.industry_id == other.industry_id
    }
}

impl Model for Industries {
    fn from_row(row: &PgRow) -> Industries
    where
        Self: Sized,
    {
        let industry_id = row.get("IndustryId");
let abbreviation = row.get("Abbreviation");
let full_name = row.get("FullName");
let status_id = row.get("StatusId");


        Self {
            industry_id,
abbreviation,
full_name,
status_id

        }
    }
}
