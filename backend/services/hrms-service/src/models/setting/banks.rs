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
pub struct Banks {
    pub bank_id: Uuid,
pub bank_name: String,
pub bank_logo: String,
pub status_id: Uuid

}

impl Banks {
    pub const TABLE: &'static str = r#""Setting"."Banks""#;
    pub const PK: &'static str = r#"BankId::TEXT=$1"#;
    pub const COLUMNS: &'static str = r#""BankId","BankName","BankLogo","StatusId""#;
    pub const COLUMNS_UPDATE: &'static str = r#""BankId"=$1,"BankName"=$2,"BankLogo"=$3,"StatusId"=$4 WHERE "BankId"=$1"#;

    pub fn get_id(&self) -> Uuid {
        self.bank_id.clone()
    }

    pub fn new(bank_id: Uuid,bank_name: String,bank_logo: String,status_id: Uuid) -> Self {
        Self {
            bank_id,
bank_name,
bank_logo,
status_id

        }
    }
}

impl PartialEq for Banks {
    fn eq(&self, other: &Self) -> bool {
        self.bank_id == other.bank_id
    }
}

impl Model for Banks {
    fn from_row(row: &PgRow) -> Banks
    where
        Self: Sized,
    {
        let bank_id = row.get("BankId");
let bank_name = row.get("BankName");
let bank_logo = row.get("BankLogo");
let status_id = row.get("StatusId");


        Self {
            bank_id,
bank_name,
bank_logo,
status_id

        }
    }
}
