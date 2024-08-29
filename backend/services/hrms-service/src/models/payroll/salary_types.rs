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
pub struct SalaryTypes {
    pub salary_type_id: Uuid,
pub full_name: String,
pub status_id: Uuid,
pub abbreviation: String

}

impl SalaryTypes {
    pub const TABLE: &'static str = r#""Payroll"."SalaryTypes""#;
    pub const PK: &'static str = r#"SalaryTypeId::TEXT=$1"#;
    pub const COLUMNS: &'static str = r#""SalaryTypeId","FullName","StatusId","Abbreviation""#;
    pub const COLUMNS_UPDATE: &'static str = r#""SalaryTypeId"=$1,"FullName"=$2,"StatusId"=$3,"Abbreviation"=$4 WHERE "SalaryTypeId"=$1"#;

    pub fn get_id(&self) -> Uuid {
        self.salary_type_id.clone()
    }

    pub fn new(salary_type_id: Uuid,full_name: String,status_id: Uuid,abbreviation: String) -> Self {
        Self {
            salary_type_id,
full_name,
status_id,
abbreviation

        }
    }
}

impl PartialEq for SalaryTypes {
    fn eq(&self, other: &Self) -> bool {
        self.salary_type_id == other.salary_type_id
    }
}

impl Model for SalaryTypes {
    fn from_row(row: &PgRow) -> SalaryTypes
    where
        Self: Sized,
    {
        let salary_type_id = row.get("SalaryTypeId");
let full_name = row.get("FullName");
let status_id = row.get("StatusId");
let abbreviation = row.get("Abbreviation");


        Self {
            salary_type_id,
full_name,
status_id,
abbreviation

        }
    }
}
