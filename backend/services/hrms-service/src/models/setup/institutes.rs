use c3k_common::interfaces::irepository::Model;
use serde::{Deserialize, Serialize};
pub use sqlx::{
    pool::PoolConnection,
    postgres::{PgArguments, PgPoolOptions, PgRow},
    types::chrono::{DateTime, Utc},
    Arguments, Error, PgPool, Postgres, Row,
};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Institutes {
    pub institute_id: Uuid,
    pub full_name: String,
    pub status_id: Uuid,
    pub abbreviation: String,
}

impl Institutes {
    pub const TABLE: &'static str = r#""Setup"."Institutes""#;
    pub const PK: &'static str = r#"instituteId::TEXT=$1"#;
    pub const COLUMNS: &'static str = r#""instituteId","FullName","StatusId","Abbreviation""#;
    pub const COLUMNS_UPDATE: &'static str =
        r#""instituteId"=$1,"FullName"=$2,"StatusId"=$3,"Abbreviation"=$4 WHERE "instituteId"=$1"#;

    pub fn get_id(&self) -> Uuid {
        self.institute_id.clone()
    }

    pub fn new(
        institute_id: Uuid,
        full_name: String,
        status_id: Uuid,
        abbreviation: String,
    ) -> Self {
        Self {
            institute_id,
            full_name,
            status_id,
            abbreviation,
        }
    }
}

impl PartialEq for Institutes {
    fn eq(&self, other: &Self) -> bool {
        self.institute_id == other.institute_id
    }
}

impl Model for Institutes {
    fn from_row(row: &PgRow) -> Institutes
    where
        Self: Sized,
    {
        let institute_id = row.get("instituteId");
        let full_name = row.get("FullName");
        let status_id = row.get("StatusId");
        let abbreviation = row.get("Abbreviation");

        Self {
            institute_id,
            full_name,
            status_id,
            abbreviation,
        }
    }
}
