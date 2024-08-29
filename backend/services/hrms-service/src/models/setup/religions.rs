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
pub struct Religions {
    pub religion_id: Uuid,
pub full_name: String,
pub status_id: Uuid,
pub abbreviation: String

}

impl Religions {
    pub const TABLE: &'static str = r#""Setup"."Religions""#;
    pub const PK: &'static str = r#"ReligionId::TEXT=$1"#;
    pub const COLUMNS: &'static str = r#""ReligionId","FullName","StatusId","Abbreviation""#;
    pub const COLUMNS_UPDATE: &'static str = r#""ReligionId"=$1,"FullName"=$2,"StatusId"=$3,"Abbreviation"=$4 WHERE "ReligionId"=$1"#;

    pub fn get_id(&self) -> Uuid {
        self.religion_id.clone()
    }

    pub fn new(religion_id: Uuid,full_name: String,status_id: Uuid,abbreviation: String) -> Self {
        Self {
            religion_id,
full_name,
status_id,
abbreviation

        }
    }
}

impl PartialEq for Religions {
    fn eq(&self, other: &Self) -> bool {
        self.religion_id == other.religion_id
    }
}

impl Model for Religions {
    fn from_row(row: &PgRow) -> Religions
    where
        Self: Sized,
    {
        let religion_id = row.get("ReligionId");
let full_name = row.get("FullName");
let status_id = row.get("StatusId");
let abbreviation = row.get("Abbreviation");


        Self {
            religion_id,
full_name,
status_id,
abbreviation

        }
    }
}
