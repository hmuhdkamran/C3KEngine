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
pub struct ApplicationStatus {
    pub application_status_id: Uuid,
pub full_name: String,
pub status_id: Uuid,
pub abbreviation: String

}

impl ApplicationStatus {
    pub const TABLE: &'static str = r#""Setup"."ApplicationStatus""#;
    pub const PK: &'static str = r#"ApplicationStatusId::TEXT=$1"#;
    pub const COLUMNS: &'static str = r#""ApplicationStatusId","FullName","StatusId","Abbreviation""#;
    pub const COLUMNS_UPDATE: &'static str = r#""ApplicationStatusId"=$1,"FullName"=$2,"StatusId"=$3,"Abbreviation"=$4 WHERE "ApplicationStatusId"=$1"#;

    pub fn get_id(&self) -> Uuid {
        self.application_status_id.clone()
    }

    pub fn new(application_status_id: Uuid,full_name: String,status_id: Uuid,abbreviation: String) -> Self {
        Self {
            application_status_id,
full_name,
status_id,
abbreviation

        }
    }
}

impl PartialEq for ApplicationStatus {
    fn eq(&self, other: &Self) -> bool {
        self.application_status_id == other.application_status_id
    }
}

impl Model for ApplicationStatus {
    fn from_row(row: &PgRow) -> ApplicationStatus
    where
        Self: Sized,
    {
        let application_status_id = row.get("ApplicationStatusId");
let full_name = row.get("FullName");
let status_id = row.get("StatusId");
let abbreviation = row.get("Abbreviation");


        Self {
            application_status_id,
full_name,
status_id,
abbreviation

        }
    }
}
