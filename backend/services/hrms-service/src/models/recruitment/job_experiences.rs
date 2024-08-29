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
pub struct JobExperiences {
    pub job_experience_id: Uuid,
pub full_name: String,
pub abbreviation: String,
pub status_id: Uuid

}

impl JobExperiences {
    pub const TABLE: &'static str = r#""Recruitment"."JobExperiences""#;
    pub const PK: &'static str = r#"JobExperienceId::TEXT=$1"#;
    pub const COLUMNS: &'static str = r#""JobExperienceId","FullName","Abbreviation","StatusId""#;
    pub const COLUMNS_UPDATE: &'static str = r#""JobExperienceId"=$1,"FullName"=$2,"Abbreviation"=$3,"StatusId"=$4 WHERE "JobExperienceId"=$1"#;

    pub fn get_id(&self) -> Uuid {
        self.job_experience_id.clone()
    }

    pub fn new(job_experience_id: Uuid,full_name: String,abbreviation: String,status_id: Uuid) -> Self {
        Self {
            job_experience_id,
full_name,
abbreviation,
status_id

        }
    }
}

impl PartialEq for JobExperiences {
    fn eq(&self, other: &Self) -> bool {
        self.job_experience_id == other.job_experience_id
    }
}

impl Model for JobExperiences {
    fn from_row(row: &PgRow) -> JobExperiences
    where
        Self: Sized,
    {
        let job_experience_id = row.get("JobExperienceId");
let full_name = row.get("FullName");
let abbreviation = row.get("Abbreviation");
let status_id = row.get("StatusId");


        Self {
            job_experience_id,
full_name,
abbreviation,
status_id

        }
    }
}
