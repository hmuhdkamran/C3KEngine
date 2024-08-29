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
pub struct EmployeeReferences {
    pub employee_reference_id: Uuid,
pub reference_name: String,
pub addres: String,
pub mobile_number: String,
pub phone_number: String,
pub personal_information_id: Uuid,
pub status_id: Uuid

}

impl EmployeeReferences {
    pub const TABLE: &'static str = r#""Employee"."EmployeeReferences""#;
    pub const PK: &'static str = r#"EmployeeReferenceId::TEXT=$1"#;
    pub const COLUMNS: &'static str = r#""EmployeeReferenceId","ReferenceName","Addres","MobileNumber","PhoneNumber","PersonalInformationId","StatusId""#;
    pub const COLUMNS_UPDATE: &'static str = r#""EmployeeReferenceId"=$1,"ReferenceName"=$2,"Addres"=$3,"MobileNumber"=$4,"PhoneNumber"=$5,"PersonalInformationId"=$6,"StatusId"=$7 WHERE "EmployeeReferenceId"=$1"#;

    pub fn get_id(&self) -> Uuid {
        self.employee_reference_id.clone()
    }

    pub fn new(employee_reference_id: Uuid,reference_name: String,addres: String,mobile_number: String,phone_number: String,personal_information_id: Uuid,status_id: Uuid) -> Self {
        Self {
            employee_reference_id,
reference_name,
addres,
mobile_number,
phone_number,
personal_information_id,
status_id

        }
    }
}

impl PartialEq for EmployeeReferences {
    fn eq(&self, other: &Self) -> bool {
        self.employee_reference_id == other.employee_reference_id
    }
}

impl Model for EmployeeReferences {
    fn from_row(row: &PgRow) -> EmployeeReferences
    where
        Self: Sized,
    {
        let employee_reference_id = row.get("EmployeeReferenceId");
let reference_name = row.get("ReferenceName");
let addres = row.get("Addres");
let mobile_number = row.get("MobileNumber");
let phone_number = row.get("PhoneNumber");
let personal_information_id = row.get("PersonalInformationId");
let status_id = row.get("StatusId");


        Self {
            employee_reference_id,
reference_name,
addres,
mobile_number,
phone_number,
personal_information_id,
status_id

        }
    }
}
