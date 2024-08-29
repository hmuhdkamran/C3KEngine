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
pub struct EmployeeExperiences {
    pub employee_experience_id: Uuid,
pub employee_id: Uuid,
pub company_name: String,
pub department_id: Uuid,
pub designation_id: Uuid,
pub contact_person: String,
pub company_email: String,
pub company_phone: String,
pub start_date: DateTime<Utc>,
pub end_date: DateTime<Utc>,
pub leaving_reason: String,
pub status_id: Uuid

}

impl EmployeeExperiences {
    pub const TABLE: &'static str = r#""Employee"."EmployeeExperiences""#;
    pub const PK: &'static str = r#"EmployeeExperienceId::TEXT=$1"#;
    pub const COLUMNS: &'static str = r#""EmployeeExperienceId","EmployeeId","CompanyName","DepartmentId","DesignationId","ContactPerson","CompanyEmail","CompanyPhone","StartDate","EndDate","LeavingReason","StatusId""#;
    pub const COLUMNS_UPDATE: &'static str = r#""EmployeeExperienceId"=$1,"EmployeeId"=$2,"CompanyName"=$3,"DepartmentId"=$4,"DesignationId"=$5,"ContactPerson"=$6,"CompanyEmail"=$7,"CompanyPhone"=$8,"StartDate"=$9,"EndDate"=$10,"LeavingReason"=$11,"StatusId"=$12 WHERE "EmployeeExperienceId"=$1"#;

    pub fn get_id(&self) -> Uuid {
        self.employee_experience_id.clone()
    }

    pub fn new(employee_experience_id: Uuid,employee_id: Uuid,company_name: String,department_id: Uuid,designation_id: Uuid,contact_person: String,company_email: String,company_phone: String,start_date: DateTime<Utc>,end_date: DateTime<Utc>,leaving_reason: String,status_id: Uuid) -> Self {
        Self {
            employee_experience_id,
employee_id,
company_name,
department_id,
designation_id,
contact_person,
company_email,
company_phone,
start_date,
end_date,
leaving_reason,
status_id

        }
    }
}

impl PartialEq for EmployeeExperiences {
    fn eq(&self, other: &Self) -> bool {
        self.employee_experience_id == other.employee_experience_id
    }
}

impl Model for EmployeeExperiences {
    fn from_row(row: &PgRow) -> EmployeeExperiences
    where
        Self: Sized,
    {
        let employee_experience_id = row.get("EmployeeExperienceId");
let employee_id = row.get("EmployeeId");
let company_name = row.get("CompanyName");
let department_id = row.get("DepartmentId");
let designation_id = row.get("DesignationId");
let contact_person = row.get("ContactPerson");
let company_email = row.get("CompanyEmail");
let company_phone = row.get("CompanyPhone");
let start_date = row.get("StartDate");
let end_date = row.get("EndDate");
let leaving_reason = row.get("LeavingReason");
let status_id = row.get("StatusId");


        Self {
            employee_experience_id,
employee_id,
company_name,
department_id,
designation_id,
contact_person,
company_email,
company_phone,
start_date,
end_date,
leaving_reason,
status_id

        }
    }
}
