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
pub struct Professoions {
    pub profession_id: Uuid,
pub full_name: String,
pub status_id: Uuid,
pub abbreviation: String

}

impl Professoions {
    pub const TABLE: &'static str = r#""Setup"."Professoions""#;
    pub const PK: &'static str = r#"ProfessionId::TEXT=$1"#;
    pub const COLUMNS: &'static str = r#""ProfessionId","FullName","StatusId","Abbreviation""#;
    pub const COLUMNS_UPDATE: &'static str = r#""ProfessionId"=$1,"FullName"=$2,"StatusId"=$3,"Abbreviation"=$4 WHERE "ProfessionId"=$1"#;

    pub fn get_id(&self) -> Uuid {
        self.profession_id.clone()
    }

    pub fn new(profession_id: Uuid,full_name: String,status_id: Uuid,abbreviation: String) -> Self {
        Self {
            profession_id,
full_name,
status_id,
abbreviation

        }
    }
}

impl PartialEq for Professoions {
    fn eq(&self, other: &Self) -> bool {
        self.profession_id == other.profession_id
    }
}

impl Model for Professoions {
    fn from_row(row: &PgRow) -> Professoions
    where
        Self: Sized,
    {
        let profession_id = row.get("ProfessionId");
let full_name = row.get("FullName");
let status_id = row.get("StatusId");
let abbreviation = row.get("Abbreviation");


        Self {
            profession_id,
full_name,
status_id,
abbreviation

        }
    }
}
