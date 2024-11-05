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
#[serde(rename_all = "PascalCase")]
pub struct ApplicationStatus {
    pub application_status_id: Uuid,
pub full_name: String,
pub status_id: Uuid,
pub abbreviation: String

}

impl ApplicationStatus {
    pub const TABLE: &'static str = r#""Setup"."ApplicationStatus""#;
    pub const PK: &'static str = "ApplicationStatusId";
    pub const COLUMNS_ARRAY: [&'static str; 4] = ["ApplicationStatusId","FullName","StatusId","Abbreviation"];

    pub fn get_id(&self) -> Uuid {
        self.application_status_id.clone()
    }

    pub fn get_args(&self) -> PgArguments {
        let mut args = PgArguments::default();
        let _ = args.add(self.application_status_id.clone());
let _ = args.add(self.full_name.clone());
let _ = args.add(self.status_id.clone());
let _ = args.add(self.abbreviation.clone());

        args
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
