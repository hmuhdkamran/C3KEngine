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
    pub const PK: &'static str = "EmployeeReferenceId";
    pub const COLUMNS_ARRAY: [&'static str; 7] = ["EmployeeReferenceId","ReferenceName","Addres","MobileNumber","PhoneNumber","PersonalInformationId","StatusId"];

    pub fn get_id(&self) -> Uuid {
        self.employee_reference_id.clone()
    }

    pub fn get_args(&self) -> PgArguments {
        let mut args = PgArguments::default();
        let _ = args.add(self.employee_reference_id.clone());
let _ = args.add(self.reference_name.clone());
let _ = args.add(self.addres.clone());
let _ = args.add(self.mobile_number.clone());
let _ = args.add(self.phone_number.clone());
let _ = args.add(self.personal_information_id.clone());
let _ = args.add(self.status_id.clone());

        args
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
