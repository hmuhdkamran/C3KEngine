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
pub struct ResignReasons {
    pub resign_reason_id: Uuid,
pub abbreviation: String,
pub reason: String,
pub status_id: Uuid

}

impl ResignReasons {
    pub const TABLE: &'static str = r#""Setup"."ResignReasons""#;
    pub const PK: &'static str = r#"ResignReasonId::TEXT=$1"#;
    pub const COLUMNS: &'static str = r#""ResignReasonId","Abbreviation","Reason","StatusId""#;
    pub const COLUMNS_UPDATE: &'static str = r#""ResignReasonId"=$1,"Abbreviation"=$2,"Reason"=$3,"StatusId"=$4 WHERE "ResignReasonId"=$1"#;

    pub fn get_id(&self) -> Uuid {
        self.resign_reason_id.clone()
    }

    pub fn new(resign_reason_id: Uuid,abbreviation: String,reason: String,status_id: Uuid) -> Self {
        Self {
            resign_reason_id,
abbreviation,
reason,
status_id

        }
    }
}

impl PartialEq for ResignReasons {
    fn eq(&self, other: &Self) -> bool {
        self.resign_reason_id == other.resign_reason_id
    }
}

impl Model for ResignReasons {
    fn from_row(row: &PgRow) -> ResignReasons
    where
        Self: Sized,
    {
        let resign_reason_id = row.get("ResignReasonId");
let abbreviation = row.get("Abbreviation");
let reason = row.get("Reason");
let status_id = row.get("StatusId");


        Self {
            resign_reason_id,
abbreviation,
reason,
status_id

        }
    }
}
