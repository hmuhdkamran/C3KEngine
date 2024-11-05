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
    pub const PK: &'static str = "JobPostId";
    pub const COLUMNS_ARRAY: [&'static str; 21] = ["JobPostId","Title","ShortDescription","LongDescription","Gender","AgeLimit","TotalPosition","JobStatusId","PostedDate","ExpiryDated","MinimumEducationId","ApplyBefore","IndustryId","CityId","JobShiftId","CareerLevelId","FunctionalAreaId","JobExperienceId","DepartmentId","SalaryFrom","SalaryTo"];

    pub fn get_id(&self) -> Uuid {
        self.job_post_id.clone()
    }

    pub fn get_args(&self) -> PgArguments {
        let mut args = PgArguments::default();
        let _ = args.add(self.job_post_id.clone());
let _ = args.add(self.title.clone());
let _ = args.add(self.short_description.clone());
let _ = args.add(self.long_description.clone());
let _ = args.add(self.gender.clone());
let _ = args.add(self.age_limit.clone());
let _ = args.add(self.total_position.clone());
let _ = args.add(self.job_status_id.clone());
let _ = args.add(self.posted_date.clone());
let _ = args.add(self.expiry_dated.clone());
let _ = args.add(self.minimum_education_id.clone());
let _ = args.add(self.apply_before.clone());
let _ = args.add(self.industry_id.clone());
let _ = args.add(self.city_id.clone());
let _ = args.add(self.job_shift_id.clone());
let _ = args.add(self.career_level_id.clone());
let _ = args.add(self.functional_area_id.clone());
let _ = args.add(self.job_experience_id.clone());
let _ = args.add(self.department_id.clone());
let _ = args.add(self.salary_from.clone());
let _ = args.add(self.salary_to.clone());

        args
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
