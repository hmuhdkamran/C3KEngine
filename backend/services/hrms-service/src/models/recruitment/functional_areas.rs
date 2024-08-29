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
pub struct FunctionalAreas {
    pub functional_area_id: Uuid,
pub full_name: String,
pub abbreviation: String,
pub status_id: Uuid

}

impl FunctionalAreas {
    pub const TABLE: &'static str = r#""Recruitment"."FunctionalAreas""#;
    pub const PK: &'static str = r#"FunctionalAreaId::TEXT=$1"#;
    pub const COLUMNS: &'static str = r#""FunctionalAreaId","FullName","Abbreviation","StatusId""#;
    pub const COLUMNS_UPDATE: &'static str = r#""FunctionalAreaId"=$1,"FullName"=$2,"Abbreviation"=$3,"StatusId"=$4 WHERE "FunctionalAreaId"=$1"#;

    pub fn get_id(&self) -> Uuid {
        self.functional_area_id.clone()
    }

    pub fn new(functional_area_id: Uuid,full_name: String,abbreviation: String,status_id: Uuid) -> Self {
        Self {
            functional_area_id,
full_name,
abbreviation,
status_id

        }
    }
}

impl PartialEq for FunctionalAreas {
    fn eq(&self, other: &Self) -> bool {
        self.functional_area_id == other.functional_area_id
    }
}

impl Model for FunctionalAreas {
    fn from_row(row: &PgRow) -> FunctionalAreas
    where
        Self: Sized,
    {
        let functional_area_id = row.get("FunctionalAreaId");
let full_name = row.get("FullName");
let abbreviation = row.get("Abbreviation");
let status_id = row.get("StatusId");


        Self {
            functional_area_id,
full_name,
abbreviation,
status_id

        }
    }
}
