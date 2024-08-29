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
pub struct Interests {
    pub interest_id: Uuid,
pub full_name: String,
pub status_id: Uuid,
pub abbreviation: String

}

impl Interests {
    pub const TABLE: &'static str = r#""Setup"."Interests""#;
    pub const PK: &'static str = r#"InterestId::TEXT=$1"#;
    pub const COLUMNS: &'static str = r#""InterestId","FullName","StatusId","Abbreviation""#;
    pub const COLUMNS_UPDATE: &'static str = r#""InterestId"=$1,"FullName"=$2,"StatusId"=$3,"Abbreviation"=$4 WHERE "InterestId"=$1"#;

    pub fn get_id(&self) -> Uuid {
        self.interest_id.clone()
    }

    pub fn new(interest_id: Uuid,full_name: String,status_id: Uuid,abbreviation: String) -> Self {
        Self {
            interest_id,
full_name,
status_id,
abbreviation

        }
    }
}

impl PartialEq for Interests {
    fn eq(&self, other: &Self) -> bool {
        self.interest_id == other.interest_id
    }
}

impl Model for Interests {
    fn from_row(row: &PgRow) -> Interests
    where
        Self: Sized,
    {
        let interest_id = row.get("InterestId");
let full_name = row.get("FullName");
let status_id = row.get("StatusId");
let abbreviation = row.get("Abbreviation");


        Self {
            interest_id,
full_name,
status_id,
abbreviation

        }
    }
}
