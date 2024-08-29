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
pub struct PersonalInformations {
    pub employee_id: Uuid,
pub first_name: String,
pub middle_name: String,
pub last_name: String,
pub employee_code: String,
pub picture: String,
pub blood_group_id: Uuid,
pub attendance_machine_no: String,
pub gender: String,
pub date_of_birth: DateTime<Utc>,
pub nationality_id: Uuid,
pub religion_id: Uuid,
pub martial_status: String

}

impl PersonalInformations {
    pub const TABLE: &'static str = r#""Employee"."PersonalInformations""#;
    pub const PK: &'static str = r#"EmployeeId::TEXT=$1"#;
    pub const COLUMNS: &'static str = r#""EmployeeId","FirstName","MiddleName","LastName","EmployeeCode","Picture","BloodGroupId","AttendanceMachineNo","Gender","DateOfBirth","NationalityId","ReligionId","MartialStatus""#;
    pub const COLUMNS_UPDATE: &'static str = r#""EmployeeId"=$1,"FirstName"=$2,"MiddleName"=$3,"LastName"=$4,"EmployeeCode"=$7,"Picture"=$21,"BloodGroupId"=$22,"AttendanceMachineNo"=$24,"Gender"=$25,"DateOfBirth"=$28,"NationalityId"=$29,"ReligionId"=$30,"MartialStatus"=$31 WHERE "EmployeeId"=$1"#;

    pub fn get_id(&self) -> Uuid {
        self.employee_id.clone()
    }

    pub fn new(employee_id: Uuid,first_name: String,middle_name: String,last_name: String,employee_code: String,picture: String,blood_group_id: Uuid,attendance_machine_no: String,gender: String,date_of_birth: DateTime<Utc>,nationality_id: Uuid,religion_id: Uuid,martial_status: String) -> Self {
        Self {
            employee_id,
first_name,
middle_name,
last_name,
employee_code,
picture,
blood_group_id,
attendance_machine_no,
gender,
date_of_birth,
nationality_id,
religion_id,
martial_status

        }
    }
}

impl PartialEq for PersonalInformations {
    fn eq(&self, other: &Self) -> bool {
        self.employee_id == other.employee_id
    }
}

impl Model for PersonalInformations {
    fn from_row(row: &PgRow) -> PersonalInformations
    where
        Self: Sized,
    {
        let employee_id = row.get("EmployeeId");
let first_name = row.get("FirstName");
let middle_name = row.get("MiddleName");
let last_name = row.get("LastName");
let employee_code = row.get("EmployeeCode");
let picture = row.get("Picture");
let blood_group_id = row.get("BloodGroupId");
let attendance_machine_no = row.get("AttendanceMachineNo");
let gender = row.get("Gender");
let date_of_birth = row.get("DateOfBirth");
let nationality_id = row.get("NationalityId");
let religion_id = row.get("ReligionId");
let martial_status = row.get("MartialStatus");


        Self {
            employee_id,
first_name,
middle_name,
last_name,
employee_code,
picture,
blood_group_id,
attendance_machine_no,
gender,
date_of_birth,
nationality_id,
religion_id,
martial_status

        }
    }
}
