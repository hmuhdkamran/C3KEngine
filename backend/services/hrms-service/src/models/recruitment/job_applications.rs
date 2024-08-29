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
pub struct JobApplications {
    pub job_application_id: Uuid,
pub candidate_id: Uuid,
pub application_date: DateTime<Utc>,
pub application_status_id: Uuid,
pub status_id: Uuid,
pub job_post_id: Uuid,
pub resume_url: String

}

impl JobApplications {
    pub const TABLE: &'static str = r#""Recruitment"."JobApplications""#;
    pub const PK: &'static str = r#"JobApplicationId::TEXT=$1"#;
    pub const COLUMNS: &'static str = r#""JobApplicationId","CandidateId","ApplicationDate","ApplicationStatusId","StatusId","JobPostId","ResumeUrl""#;
    pub const COLUMNS_UPDATE: &'static str = r#""JobApplicationId"=$1,"CandidateId"=$2,"ApplicationDate"=$3,"ApplicationStatusId"=$4,"StatusId"=$5,"JobPostId"=$6,"ResumeUrl"=$7 WHERE "JobApplicationId"=$1"#;

    pub fn get_id(&self) -> Uuid {
        self.job_application_id.clone()
    }

    pub fn new(job_application_id: Uuid,candidate_id: Uuid,application_date: DateTime<Utc>,application_status_id: Uuid,status_id: Uuid,job_post_id: Uuid,resume_url: String) -> Self {
        Self {
            job_application_id,
candidate_id,
application_date,
application_status_id,
status_id,
job_post_id,
resume_url

        }
    }
}

impl PartialEq for JobApplications {
    fn eq(&self, other: &Self) -> bool {
        self.job_application_id == other.job_application_id
    }
}

impl Model for JobApplications {
    fn from_row(row: &PgRow) -> JobApplications
    where
        Self: Sized,
    {
        let job_application_id = row.get("JobApplicationId");
let candidate_id = row.get("CandidateId");
let application_date = row.get("ApplicationDate");
let application_status_id = row.get("ApplicationStatusId");
let status_id = row.get("StatusId");
let job_post_id = row.get("JobPostId");
let resume_url = row.get("ResumeUrl");


        Self {
            job_application_id,
candidate_id,
application_date,
application_status_id,
status_id,
job_post_id,
resume_url

        }
    }
}
