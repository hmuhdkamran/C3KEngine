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
pub struct RecruitmentJobStatuses {
    pub job_status_id: Uuid,
pub full_name: String,
pub abbreviation: String,
pub status_id: Uuid

}

impl RecruitmentJobStatuses {
    pub const TABLE: &'static str = r#""Recruitment"."RecruitmentJobStatuses""#;
    pub const PK: &'static str = r#"JobStatusId::TEXT=$1"#;
    pub const COLUMNS: &'static str = r#""JobStatusId","FullName","Abbreviation","StatusId""#;
    pub const COLUMNS_UPDATE: &'static str = r#""JobStatusId"=$1,"FullName"=$2,"Abbreviation"=$3,"StatusId"=$4 WHERE "JobStatusId"=$1"#;

    pub fn get_id(&self) -> Uuid {
        self.job_status_id.clone()
    }

    pub fn new(job_status_id: Uuid,full_name: String,abbreviation: String,status_id: Uuid) -> Self {
        Self {
            job_status_id,
full_name,
abbreviation,
status_id

        }
    }
}

impl PartialEq for RecruitmentJobStatuses {
    fn eq(&self, other: &Self) -> bool {
        self.job_status_id == other.job_status_id
    }
}

impl Model for RecruitmentJobStatuses {
    fn from_row(row: &PgRow) -> RecruitmentJobStatuses
    where
        Self: Sized,
    {
        let job_status_id = row.get("JobStatusId");
let full_name = row.get("FullName");
let abbreviation = row.get("Abbreviation");
let status_id = row.get("StatusId");


        Self {
            job_status_id,
full_name,
abbreviation,
status_id

        }
    }
}
