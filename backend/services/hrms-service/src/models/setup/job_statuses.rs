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
pub struct JobStatuses {
    pub job_status_id: Uuid,
pub abbreviation: String,
pub full_name: String,
pub status_id: Uuid

}

impl JobStatuses {
    pub const TABLE: &'static str = r#""Setup"."JobStatuses""#;
    pub const PK: &'static str = "JobStatusId";
    pub const COLUMNS_ARRAY: [&'static str; 4] = ["JobStatusId","Abbreviation","FullName","StatusId"];

    pub fn get_id(&self) -> Uuid {
        self.job_status_id.clone()
    }

    pub fn get_args(&self) -> PgArguments {
        let mut args = PgArguments::default();
        let _ = args.add(self.job_status_id.clone());
let _ = args.add(self.abbreviation.clone());
let _ = args.add(self.full_name.clone());
let _ = args.add(self.status_id.clone());

        args
    }

    pub fn new(job_status_id: Uuid,abbreviation: String,full_name: String,status_id: Uuid) -> Self {
        Self {
            job_status_id,
abbreviation,
full_name,
status_id

        }
    }
}

impl PartialEq for JobStatuses {
    fn eq(&self, other: &Self) -> bool {
        self.job_status_id == other.job_status_id
    }
}

impl Model for JobStatuses {
    fn from_row(row: &PgRow) -> JobStatuses
    where
        Self: Sized,
    {
        let job_status_id = row.get("JobStatusId");
let abbreviation = row.get("Abbreviation");
let full_name = row.get("FullName");
let status_id = row.get("StatusId");


        Self {
            job_status_id,
abbreviation,
full_name,
status_id

        }
    }
}
