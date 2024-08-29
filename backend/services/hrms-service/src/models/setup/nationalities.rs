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
pub struct Nationalities {
    pub nationality_id: Uuid,
pub full_name: String,
pub status_id: Uuid,
pub abbreviation: String

}

impl Nationalities {
    pub const TABLE: &'static str = r#""Setup"."Nationalities""#;
    pub const PK: &'static str = r#"NationalityId::TEXT=$1"#;
    pub const COLUMNS: &'static str = r#""NationalityId","FullName","StatusId","Abbreviation""#;
    pub const COLUMNS_UPDATE: &'static str = r#""NationalityId"=$1,"FullName"=$2,"StatusId"=$3,"Abbreviation"=$4 WHERE "NationalityId"=$1"#;

    pub fn get_id(&self) -> Uuid {
        self.nationality_id.clone()
    }

    pub fn new(nationality_id: Uuid,full_name: String,status_id: Uuid,abbreviation: String) -> Self {
        Self {
            nationality_id,
full_name,
status_id,
abbreviation

        }
    }
}

impl PartialEq for Nationalities {
    fn eq(&self, other: &Self) -> bool {
        self.nationality_id == other.nationality_id
    }
}

impl Model for Nationalities {
    fn from_row(row: &PgRow) -> Nationalities
    where
        Self: Sized,
    {
        let nationality_id = row.get("NationalityId");
let full_name = row.get("FullName");
let status_id = row.get("StatusId");
let abbreviation = row.get("Abbreviation");


        Self {
            nationality_id,
full_name,
status_id,
abbreviation

        }
    }
}
