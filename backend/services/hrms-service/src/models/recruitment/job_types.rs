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
pub struct JobTypes {
    pub job_type_id: Uuid,
pub full_name: String,
pub abbreviation: String,
pub status_id: Uuid

}

impl JobTypes {
    pub const TABLE: &'static str = r#""Recruitment"."JobTypes""#;
    pub const PK: &'static str = "JobTypeId";
    pub const COLUMNS_ARRAY: [&'static str; 4] = ["JobTypeId","FullName","Abbreviation","StatusId"];

    pub fn get_id(&self) -> Uuid {
        self.job_type_id.clone()
    }

    pub fn get_args(&self) -> PgArguments {
        let mut args = PgArguments::default();
        let _ = args.add(self.job_type_id.clone());
let _ = args.add(self.full_name.clone());
let _ = args.add(self.abbreviation.clone());
let _ = args.add(self.status_id.clone());

        args
    }

    pub fn new(job_type_id: Uuid,full_name: String,abbreviation: String,status_id: Uuid) -> Self {
        Self {
            job_type_id,
full_name,
abbreviation,
status_id

        }
    }
}

impl PartialEq for JobTypes {
    fn eq(&self, other: &Self) -> bool {
        self.job_type_id == other.job_type_id
    }
}

impl Model for JobTypes {
    fn from_row(row: &PgRow) -> JobTypes
    where
        Self: Sized,
    {
        let job_type_id = row.get("JobTypeId");
let full_name = row.get("FullName");
let abbreviation = row.get("Abbreviation");
let status_id = row.get("StatusId");


        Self {
            job_type_id,
full_name,
abbreviation,
status_id

        }
    }
}
