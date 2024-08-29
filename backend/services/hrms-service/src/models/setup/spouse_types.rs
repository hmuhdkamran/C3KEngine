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
pub struct SpouseTypes {
    pub spouse_type_id: Uuid,
pub abbreviation: String,
pub full_name: String,
pub status_id: Uuid

}

impl SpouseTypes {
    pub const TABLE: &'static str = r#""Setup"."SpouseTypes""#;
    pub const PK: &'static str = r#"SpouseTypeId::TEXT=$1"#;
    pub const COLUMNS: &'static str = r#""SpouseTypeId","Abbreviation","FullName","StatusId""#;
    pub const COLUMNS_UPDATE: &'static str = r#""SpouseTypeId"=$1,"Abbreviation"=$2,"FullName"=$3,"StatusId"=$4 WHERE "SpouseTypeId"=$1"#;

    pub fn get_id(&self) -> Uuid {
        self.spouse_type_id.clone()
    }

    pub fn new(spouse_type_id: Uuid,abbreviation: String,full_name: String,status_id: Uuid) -> Self {
        Self {
            spouse_type_id,
abbreviation,
full_name,
status_id

        }
    }
}

impl PartialEq for SpouseTypes {
    fn eq(&self, other: &Self) -> bool {
        self.spouse_type_id == other.spouse_type_id
    }
}

impl Model for SpouseTypes {
    fn from_row(row: &PgRow) -> SpouseTypes
    where
        Self: Sized,
    {
        let spouse_type_id = row.get("SpouseTypeId");
let abbreviation = row.get("Abbreviation");
let full_name = row.get("FullName");
let status_id = row.get("StatusId");


        Self {
            spouse_type_id,
abbreviation,
full_name,
status_id

        }
    }
}
