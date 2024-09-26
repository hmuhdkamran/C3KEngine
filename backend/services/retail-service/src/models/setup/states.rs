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
pub struct States {
    pub state_id: Uuid,
pub abbreviation: String,
pub full_name: String,
pub country_id: Uuid,
pub status_id: Uuid

}

impl States {
    pub const TABLE: &'static str = r#""Setup"."States""#;
    pub const PK: &'static str = r#"StateId::TEXT=$1"#;
    pub const COLUMNS: &'static str = r#""StateId","Abbreviation","FullName","CountryId","StatusId""#;
    pub const COLUMNS_UPDATE: &'static str = r#""StateId"=$1,"Abbreviation"=$2,"FullName"=$3,"CountryId"=$4,"StatusId"=$5 WHERE "StateId"=$1"#;

    pub fn get_id(&self) -> Uuid {
        self.state_id.clone()
    }

    pub fn new(state_id: Uuid,abbreviation: String,full_name: String,country_id: Uuid,status_id: Uuid) -> Self {
        Self {
            state_id,
abbreviation,
full_name,
country_id,
status_id

        }
    }
}

impl PartialEq for States {
    fn eq(&self, other: &Self) -> bool {
        self.state_id == other.state_id
    }
}

impl Model for States {
    fn from_row(row: &PgRow) -> States
    where
        Self: Sized,
    {
        let state_id = row.get("StateId");
let abbreviation = row.get("Abbreviation");
let full_name = row.get("FullName");
let country_id = row.get("CountryId");
let status_id = row.get("StatusId");


        Self {
            state_id,
abbreviation,
full_name,
country_id,
status_id

        }
    }
}
