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
pub struct JobPosts {
    pub job_post_id: Uuid,
pub title: String,
pub short_description: String,
pub long_description: String,
pub gender: String,
pub age_limit: i32,
pub total_position: i32,
pub job_status_id: Uuid,
pub posted_date: DateTime<Utc>,
pub expiry_dated: DateTime<Utc>,
pub minimum_education_id: Uuid,
pub apply_before: DateTime<Utc>,
pub industry_id: Uuid,
pub city_id: Uuid,
pub job_shift_id: Uuid,
pub career_level_id: Uuid,
pub functional_area_id: Uuid,
pub job_experience_id: Uuid,
pub department_id: Uuid,
pub salary_from: f64,
pub salary_to: f64

}

impl JobPosts {
    pub const TABLE: &'static str = r#""Recruitment"."JobPosts""#;
    pub const PK: &'static str = r#"JobPostId::TEXT=$1"#;
    pub const COLUMNS: &'static str = r#""JobPostId","Title","ShortDescription","LongDescription","Gender","AgeLimit","TotalPosition","JobStatusId","PostedDate","ExpiryDated","MinimumEducationId","ApplyBefore","IndustryId","CityId","JobShiftId","CareerLevelId","FunctionalAreaId","JobExperienceId","DepartmentId","SalaryFrom","SalaryTo""#;
    pub const COLUMNS_UPDATE: &'static str = r#""JobPostId"=$1,"Title"=$2,"ShortDescription"=$3,"LongDescription"=$4,"Gender"=$7,"AgeLimit"=$8,"TotalPosition"=$9,"JobStatusId"=$11,"PostedDate"=$12,"ExpiryDated"=$13,"MinimumEducationId"=$14,"ApplyBefore"=$15,"IndustryId"=$17,"CityId"=$18,"JobShiftId"=$19,"CareerLevelId"=$20,"FunctionalAreaId"=$21,"JobExperienceId"=$22,"DepartmentId"=$23,"SalaryFrom"=$24,"SalaryTo"=$25 WHERE "JobPostId"=$1"#;

    pub fn get_id(&self) -> Uuid {
        self.job_post_id.clone()
    }

    pub fn new(job_post_id: Uuid,title: String,short_description: String,long_description: String,gender: String,age_limit: i32,total_position: i32,job_status_id: Uuid,posted_date: DateTime<Utc>,expiry_dated: DateTime<Utc>,minimum_education_id: Uuid,apply_before: DateTime<Utc>,industry_id: Uuid,city_id: Uuid,job_shift_id: Uuid,career_level_id: Uuid,functional_area_id: Uuid,job_experience_id: Uuid,department_id: Uuid,salary_from: f64,salary_to: f64) -> Self {
        Self {
            job_post_id,
title,
short_description,
long_description,
gender,
age_limit,
total_position,
job_status_id,
posted_date,
expiry_dated,
minimum_education_id,
apply_before,
industry_id,
city_id,
job_shift_id,
career_level_id,
functional_area_id,
job_experience_id,
department_id,
salary_from,
salary_to

        }
    }
}

impl PartialEq for JobPosts {
    fn eq(&self, other: &Self) -> bool {
        self.job_post_id == other.job_post_id
    }
}

impl Model for JobPosts {
    fn from_row(row: &PgRow) -> JobPosts
    where
        Self: Sized,
    {
        let job_post_id = row.get("JobPostId");
let title = row.get("Title");
let short_description = row.get("ShortDescription");
let long_description = row.get("LongDescription");
let gender = row.get("Gender");
let age_limit = row.get("AgeLimit");
let total_position = row.get("TotalPosition");
let job_status_id = row.get("JobStatusId");
let posted_date = row.get("PostedDate");
let expiry_dated = row.get("ExpiryDated");
let minimum_education_id = row.get("MinimumEducationId");
let apply_before = row.get("ApplyBefore");
let industry_id = row.get("IndustryId");
let city_id = row.get("CityId");
let job_shift_id = row.get("JobShiftId");
let career_level_id = row.get("CareerLevelId");
let functional_area_id = row.get("FunctionalAreaId");
let job_experience_id = row.get("JobExperienceId");
let department_id = row.get("DepartmentId");
let salary_from = row.get("SalaryFrom");
let salary_to = row.get("SalaryTo");


        Self {
            job_post_id,
title,
short_description,
long_description,
gender,
age_limit,
total_position,
job_status_id,
posted_date,
expiry_dated,
minimum_education_id,
apply_before,
industry_id,
city_id,
job_shift_id,
career_level_id,
functional_area_id,
job_experience_id,
department_id,
salary_from,
salary_to

        }
    }
}
