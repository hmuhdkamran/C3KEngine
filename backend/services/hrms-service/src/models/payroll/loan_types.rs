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
pub struct LoanTypes {
    pub loan_type_id: Uuid,
pub full_name: String,
pub status_id: Uuid,
pub abbreviation: String

}

impl LoanTypes {
    pub const TABLE: &'static str = r#""Payroll"."LoanTypes""#;
    pub const PK: &'static str = r#"LoanTypeId::TEXT=$1"#;
    pub const COLUMNS: &'static str = r#""LoanTypeId","FullName","StatusId","Abbreviation""#;
    pub const COLUMNS_UPDATE: &'static str = r#""LoanTypeId"=$1,"FullName"=$2,"StatusId"=$3,"Abbreviation"=$4 WHERE "LoanTypeId"=$1"#;

    pub fn get_id(&self) -> Uuid {
        self.loan_type_id.clone()
    }

    pub fn new(loan_type_id: Uuid,full_name: String,status_id: Uuid,abbreviation: String) -> Self {
        Self {
            loan_type_id,
full_name,
status_id,
abbreviation

        }
    }
}

impl PartialEq for LoanTypes {
    fn eq(&self, other: &Self) -> bool {
        self.loan_type_id == other.loan_type_id
    }
}

impl Model for LoanTypes {
    fn from_row(row: &PgRow) -> LoanTypes
    where
        Self: Sized,
    {
        let loan_type_id = row.get("LoanTypeId");
let full_name = row.get("FullName");
let status_id = row.get("StatusId");
let abbreviation = row.get("Abbreviation");


        Self {
            loan_type_id,
full_name,
status_id,
abbreviation

        }
    }
}
