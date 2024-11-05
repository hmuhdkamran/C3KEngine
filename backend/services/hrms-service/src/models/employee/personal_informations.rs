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
    pub const PK: &'static str = "EmployeeId";
    pub const COLUMNS_ARRAY: [&'static str; 13] = ["EmployeeId","FirstName","MiddleName","LastName","EmployeeCode","Picture","BloodGroupId","AttendanceMachineNo","Gender","DateOfBirth","NationalityId","ReligionId","MartialStatus"];

    pub fn get_id(&self) -> Uuid {
        self.employee_id.clone()
    }

    pub fn get_args(&self) -> PgArguments {
        let mut args = PgArguments::default();
        let _ = args.add(self.employee_id.clone());
let _ = args.add(self.first_name.clone());
let _ = args.add(self.middle_name.clone());
let _ = args.add(self.last_name.clone());
let _ = args.add(self.employee_code.clone());
let _ = args.add(self.picture.clone());
let _ = args.add(self.blood_group_id.clone());
let _ = args.add(self.attendance_machine_no.clone());
let _ = args.add(self.gender.clone());
let _ = args.add(self.date_of_birth.clone());
let _ = args.add(self.nationality_id.clone());
let _ = args.add(self.religion_id.clone());
let _ = args.add(self.martial_status.clone());

        args
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
