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
pub struct Months {
    pub month_id: Uuid,
pub month_no: i32,
pub status_id: Uuid

}

impl Months {
    pub const TABLE: &'static str = r#""Setting"."Months""#;
    pub const PK: &'static str = r#"MonthId::TEXT=$1"#;
    pub const COLUMNS: &'static str = r#""MonthId","MonthNo","StatusId""#;
    pub const COLUMNS_UPDATE: &'static str = r#""MonthId"=$1,"MonthNo"=$2,"StatusId"=$3 WHERE "MonthId"=$1"#;

    pub fn get_id(&self) -> Uuid {
        self.month_id.clone()
    }

    pub fn new(month_id: Uuid,month_no: i32,status_id: Uuid) -> Self {
        Self {
            month_id,
month_no,
status_id

        }
    }
}

impl PartialEq for Months {
    fn eq(&self, other: &Self) -> bool {
        self.month_id == other.month_id
    }
}

impl Model for Months {
    fn from_row(row: &PgRow) -> Months
    where
        Self: Sized,
    {
        let month_id = row.get("MonthId");
let month_no = row.get("MonthNo");
let status_id = row.get("StatusId");


        Self {
            month_id,
month_no,
status_id

        }
    }
}
