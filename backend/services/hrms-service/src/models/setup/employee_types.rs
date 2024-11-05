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
pub struct EmployeeTypes {
    pub employee_type_id: Uuid,
pub full_name: String,
pub status_id: Uuid,
pub abbreviation: String

}

impl EmployeeTypes {
    pub const TABLE: &'static str = r#""Setup"."EmployeeTypes""#;
    pub const PK: &'static str = "EmployeeTypeId";
    pub const COLUMNS_ARRAY: [&'static str; 4] = ["EmployeeTypeId","FullName","StatusId","Abbreviation"];

    pub fn get_id(&self) -> Uuid {
        self.employee_type_id.clone()
    }

    pub fn get_args(&self) -> PgArguments {
        let mut args = PgArguments::default();
        let _ = args.add(self.employee_type_id.clone());
let _ = args.add(self.full_name.clone());
let _ = args.add(self.status_id.clone());
let _ = args.add(self.abbreviation.clone());

        args
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
