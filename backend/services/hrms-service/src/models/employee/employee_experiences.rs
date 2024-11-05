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
    pub const PK: &'static str = "EmployeeExperienceId";
    pub const COLUMNS_ARRAY: [&'static str; 12] = ["EmployeeExperienceId","EmployeeId","CompanyName","DepartmentId","DesignationId","ContactPerson","CompanyEmail","CompanyPhone","StartDate","EndDate","LeavingReason","StatusId"];

    pub fn get_id(&self) -> Uuid {
        self.employee_experience_id.clone()
    }

    pub fn get_args(&self) -> PgArguments {
        let mut args = PgArguments::default();
        let _ = args.add(self.employee_experience_id.clone());
let _ = args.add(self.employee_id.clone());
let _ = args.add(self.company_name.clone());
let _ = args.add(self.department_id.clone());
let _ = args.add(self.designation_id.clone());
let _ = args.add(self.contact_person.clone());
let _ = args.add(self.company_email.clone());
let _ = args.add(self.company_phone.clone());
let _ = args.add(self.start_date.clone());
let _ = args.add(self.end_date.clone());
let _ = args.add(self.leaving_reason.clone());
let _ = args.add(self.status_id.clone());

        args
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
