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
pub struct EmployeeBankInfos {
    pub employee_bank_info_id: Uuid,
pub employee_id: Uuid,
pub bank_branch_id: Uuid,
pub account_title: String,
pub account_number: String,
pub status_id: Uuid

}

impl EmployeeBankInfos {
    pub const TABLE: &'static str = r#""Employee"."EmployeeBankInfos""#;
    pub const PK: &'static str = r#"EmployeeBankInfoId::TEXT=$1"#;
    pub const COLUMNS: &'static str = r#""EmployeeBankInfoId","EmployeeId","BankBranchId","AccountTitle","AccountNumber","StatusId""#;
    pub const COLUMNS_UPDATE: &'static str = r#""EmployeeBankInfoId"=$1,"EmployeeId"=$2,"BankBranchId"=$3,"AccountTitle"=$4,"AccountNumber"=$5,"StatusId"=$6 WHERE "EmployeeBankInfoId"=$1"#;

    pub fn get_id(&self) -> Uuid {
        self.employee_bank_info_id.clone()
    }

    pub fn new(employee_bank_info_id: Uuid,employee_id: Uuid,bank_branch_id: Uuid,account_title: String,account_number: String,status_id: Uuid) -> Self {
        Self {
            employee_bank_info_id,
employee_id,
bank_branch_id,
account_title,
account_number,
status_id

        }
    }
}

impl PartialEq for EmployeeBankInfos {
    fn eq(&self, other: &Self) -> bool {
        self.employee_bank_info_id == other.employee_bank_info_id
    }
}

impl Model for EmployeeBankInfos {
    fn from_row(row: &PgRow) -> EmployeeBankInfos
    where
        Self: Sized,
    {
        let employee_bank_info_id = row.get("EmployeeBankInfoId");
let employee_id = row.get("EmployeeId");
let bank_branch_id = row.get("BankBranchId");
let account_title = row.get("AccountTitle");
let account_number = row.get("AccountNumber");
let status_id = row.get("StatusId");


        Self {
            employee_bank_info_id,
employee_id,
bank_branch_id,
account_title,
account_number,
status_id

        }
    }
}
