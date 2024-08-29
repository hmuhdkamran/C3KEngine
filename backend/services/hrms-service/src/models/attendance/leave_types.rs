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
pub struct LeaveTypes {
    pub leave_type_id: Uuid,
pub abbreviation: String,
pub full_name: String,
pub status_id: Uuid

}

impl LeaveTypes {
    pub const TABLE: &'static str = r#""Attendance"."LeaveTypes""#;
    pub const PK: &'static str = r#"LeaveTypeId::TEXT=$1"#;
    pub const COLUMNS: &'static str = r#""LeaveTypeId","Abbreviation","FullName","StatusId""#;
    pub const COLUMNS_UPDATE: &'static str = r#""LeaveTypeId"=$1,"Abbreviation"=$2,"FullName"=$3,"StatusId"=$4 WHERE "LeaveTypeId"=$1"#;

    pub fn get_id(&self) -> Uuid {
        self.leave_type_id.clone()
    }

    pub fn new(leave_type_id: Uuid,abbreviation: String,full_name: String,status_id: Uuid) -> Self {
        Self {
            leave_type_id,
abbreviation,
full_name,
status_id

        }
    }
}

impl PartialEq for LeaveTypes {
    fn eq(&self, other: &Self) -> bool {
        self.leave_type_id == other.leave_type_id
    }
}

impl Model for LeaveTypes {
    fn from_row(row: &PgRow) -> LeaveTypes
    where
        Self: Sized,
    {
        let leave_type_id = row.get("LeaveTypeId");
let abbreviation = row.get("Abbreviation");
let full_name = row.get("FullName");
let status_id = row.get("StatusId");


        Self {
            leave_type_id,
abbreviation,
full_name,
status_id

        }
    }
}
