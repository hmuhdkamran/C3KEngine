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
pub struct EmployeeEducations {
    pub employee_education_id: Uuid,
pub degree_id: Uuid,
pub major_subject: String,
pub completion_year: i32,
pub institute_id: Uuid,
pub personal_information_id: Uuid,
pub status_id: Uuid

}

impl EmployeeEducations {
    pub const TABLE: &'static str = r#""Employee"."EmployeeEducations""#;
    pub const PK: &'static str = r#"EmployeeEducationId::TEXT=$1"#;
    pub const COLUMNS: &'static str = r#""EmployeeEducationId","DegreeId","MajorSubject","CompletionYear","InstituteId","PersonalInformationId","StatusId""#;
    pub const COLUMNS_UPDATE: &'static str = r#""EmployeeEducationId"=$1,"DegreeId"=$2,"MajorSubject"=$3,"CompletionYear"=$4,"InstituteId"=$5,"PersonalInformationId"=$6,"StatusId"=$7 WHERE "EmployeeEducationId"=$1"#;

    pub fn get_id(&self) -> Uuid {
        self.employee_education_id.clone()
    }

    pub fn new(employee_education_id: Uuid,degree_id: Uuid,major_subject: String,completion_year: i32,institute_id: Uuid,personal_information_id: Uuid,status_id: Uuid) -> Self {
        Self {
            employee_education_id,
degree_id,
major_subject,
completion_year,
institute_id,
personal_information_id,
status_id

        }
    }
}

impl PartialEq for EmployeeEducations {
    fn eq(&self, other: &Self) -> bool {
        self.employee_education_id == other.employee_education_id
    }
}

impl Model for EmployeeEducations {
    fn from_row(row: &PgRow) -> EmployeeEducations
    where
        Self: Sized,
    {
        let employee_education_id = row.get("EmployeeEducationId");
let degree_id = row.get("DegreeId");
let major_subject = row.get("MajorSubject");
let completion_year = row.get("CompletionYear");
let institute_id = row.get("InstituteId");
let personal_information_id = row.get("PersonalInformationId");
let status_id = row.get("StatusId");


        Self {
            employee_education_id,
degree_id,
major_subject,
completion_year,
institute_id,
personal_information_id,
status_id

        }
    }
}
