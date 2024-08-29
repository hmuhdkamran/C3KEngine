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
pub struct BankBranches {
    pub bank_branch_id: Uuid,
pub bank_id: Uuid,
pub branch_code: String,
pub branch_name: String,
pub branch_address: String,
pub status_id: Uuid

}

impl BankBranches {
    pub const TABLE: &'static str = r#""Setting"."BankBranches""#;
    pub const PK: &'static str = r#"BankBranchId::TEXT=$1"#;
    pub const COLUMNS: &'static str = r#""BankBranchId","BankId","BranchCode","BranchName","BranchAddress","StatusId""#;
    pub const COLUMNS_UPDATE: &'static str = r#""BankBranchId"=$1,"BankId"=$2,"BranchCode"=$3,"BranchName"=$4,"BranchAddress"=$5,"StatusId"=$6 WHERE "BankBranchId"=$1"#;

    pub fn get_id(&self) -> Uuid {
        self.bank_branch_id.clone()
    }

    pub fn new(bank_branch_id: Uuid,bank_id: Uuid,branch_code: String,branch_name: String,branch_address: String,status_id: Uuid) -> Self {
        Self {
            bank_branch_id,
bank_id,
branch_code,
branch_name,
branch_address,
status_id

        }
    }
}

impl PartialEq for BankBranches {
    fn eq(&self, other: &Self) -> bool {
        self.bank_branch_id == other.bank_branch_id
    }
}

impl Model for BankBranches {
    fn from_row(row: &PgRow) -> BankBranches
    where
        Self: Sized,
    {
        let bank_branch_id = row.get("BankBranchId");
let bank_id = row.get("BankId");
let branch_code = row.get("BranchCode");
let branch_name = row.get("BranchName");
let branch_address = row.get("BranchAddress");
let status_id = row.get("StatusId");


        Self {
            bank_branch_id,
bank_id,
branch_code,
branch_name,
branch_address,
status_id

        }
    }
}
