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
pub struct EmployeeTypes {
    pub employee_type_id: Uuid,
pub full_name: String,
pub status_id: Uuid,
pub abbreviation: String

}

impl EmployeeTypes {
    pub const TABLE: &'static str = r#""Setup"."EmployeeTypes""#;
    pub const PK: &'static str = r#"EmployeeTypeId::TEXT=$1"#;
    pub const COLUMNS: &'static str = r#""EmployeeTypeId","FullName","StatusId","Abbreviation""#;
    pub const COLUMNS_UPDATE: &'static str = r#""EmployeeTypeId"=$1,"FullName"=$2,"StatusId"=$3,"Abbreviation"=$4 WHERE "EmployeeTypeId"=$1"#;

    pub fn get_id(&self) -> Uuid {
        self.employee_type_id.clone()
    }

    pub fn new(employee_type_id: Uuid,full_name: String,status_id: Uuid,abbreviation: String) -> Self {
        Self {
            employee_type_id,
full_name,
status_id,
abbreviation

        }
    }
}

impl PartialEq for EmployeeTypes {
    fn eq(&self, other: &Self) -> bool {
        self.employee_type_id == other.employee_type_id
    }
}

impl Model for EmployeeTypes {
    fn from_row(row: &PgRow) -> EmployeeTypes
    where
        Self: Sized,
    {
        let employee_type_id = row.get("EmployeeTypeId");
let full_name = row.get("FullName");
let status_id = row.get("StatusId");
let abbreviation = row.get("Abbreviation");


        Self {
            employee_type_id,
full_name,
status_id,
abbreviation

        }
    }
}
