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
pub struct Degrees {
    pub degree_id: Uuid,
pub full_name: String,
pub status_id: Uuid,
pub abbreviation: String,
pub label: String

}

impl Degrees {
    pub const TABLE: &'static str = r#""Setup"."Degrees""#;
    pub const PK: &'static str = r#"DegreeId::TEXT=$1"#;
    pub const COLUMNS: &'static str = r#""DegreeId","FullName","StatusId","Abbreviation","Label""#;
    pub const COLUMNS_UPDATE: &'static str = r#""DegreeId"=$1,"FullName"=$2,"StatusId"=$3,"Abbreviation"=$4,"Label"=$5 WHERE "DegreeId"=$1"#;

    pub fn get_id(&self) -> Uuid {
        self.degree_id.clone()
    }

    pub fn new(degree_id: Uuid,full_name: String,status_id: Uuid,abbreviation: String,label: String) -> Self {
        Self {
            degree_id,
full_name,
status_id,
abbreviation,
label

        }
    }
}

impl PartialEq for Degrees {
    fn eq(&self, other: &Self) -> bool {
        self.degree_id == other.degree_id
    }
}

impl Model for Degrees {
    fn from_row(row: &PgRow) -> Degrees
    where
        Self: Sized,
    {
        let degree_id = row.get("DegreeId");
let full_name = row.get("FullName");
let status_id = row.get("StatusId");
let abbreviation = row.get("Abbreviation");
let label = row.get("Label");


        Self {
            degree_id,
full_name,
status_id,
abbreviation,
label

        }
    }
}
