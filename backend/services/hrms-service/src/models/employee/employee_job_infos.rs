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
pub struct EmployeeJobInfos {
    pub employee_job_info_id: Uuid,
pub employee_id: Uuid,
pub department_id: Uuid,
pub designation_id: Uuid,
pub grade_id: Uuid,
pub employee_type_id: Uuid,
pub job_status_id: Uuid,
pub date_of_joining: DateTime<Utc>,
pub employee_code: String,
pub status_id: Uuid,
pub end_date: DateTime<Utc>

}

impl EmployeeJobInfos {
    pub const TABLE: &'static str = r#""Employee"."EmployeeJobInfos""#;
    pub const PK: &'static str = "EmployeeJobInfoId";
    pub const COLUMNS_ARRAY: [&'static str; 11] = ["EmployeeJobInfoId","EmployeeId","DepartmentId","DesignationId","GradeId","EmployeeTypeId","JobStatusId","DateOfJoining","EmployeeCode","StatusId","EndDate"];

    pub fn get_id(&self) -> Uuid {
        self.employee_job_info_id.clone()
    }

    pub fn get_args(&self) -> PgArguments {
        let mut args = PgArguments::default();
        let _ = args.add(self.employee_job_info_id.clone());
let _ = args.add(self.employee_id.clone());
let _ = args.add(self.department_id.clone());
let _ = args.add(self.designation_id.clone());
let _ = args.add(self.grade_id.clone());
let _ = args.add(self.employee_type_id.clone());
let _ = args.add(self.job_status_id.clone());
let _ = args.add(self.date_of_joining.clone());
let _ = args.add(self.employee_code.clone());
let _ = args.add(self.status_id.clone());
let _ = args.add(self.end_date.clone());

        args
    }

    pub fn new(employee_job_info_id: Uuid,employee_id: Uuid,department_id: Uuid,designation_id: Uuid,grade_id: Uuid,employee_type_id: Uuid,job_status_id: Uuid,date_of_joining: DateTime<Utc>,employee_code: String,status_id: Uuid,end_date: DateTime<Utc>) -> Self {
        Self {
            employee_job_info_id,
employee_id,
department_id,
designation_id,
grade_id,
employee_type_id,
job_status_id,
date_of_joining,
employee_code,
status_id,
end_date

        }
    }
}

impl PartialEq for EmployeeJobInfos {
    fn eq(&self, other: &Self) -> bool {
        self.employee_job_info_id == other.employee_job_info_id
    }
}

impl Model for EmployeeJobInfos {
    fn from_row(row: &PgRow) -> EmployeeJobInfos
    where
        Self: Sized,
    {
        let employee_job_info_id = row.get("EmployeeJobInfoId");
let employee_id = row.get("EmployeeId");
let department_id = row.get("DepartmentId");
let designation_id = row.get("DesignationId");
let grade_id = row.get("GradeId");
let employee_type_id = row.get("EmployeeTypeId");
let job_status_id = row.get("JobStatusId");
let date_of_joining = row.get("DateOfJoining");
let employee_code = row.get("EmployeeCode");
let status_id = row.get("StatusId");
let end_date = row.get("EndDate");


        Self {
            employee_job_info_id,
employee_id,
department_id,
designation_id,
grade_id,
employee_type_id,
job_status_id,
date_of_joining,
employee_code,
status_id,
end_date

        }
    }
}
