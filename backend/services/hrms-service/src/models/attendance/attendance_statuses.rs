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
pub struct AttendanceStatuses {
    pub attendance_status_id: Uuid,
pub full_name: String,
pub abbreviation: String,
pub status_id: Uuid

}

impl AttendanceStatuses {
    pub const TABLE: &'static str = r#""Attendance"."AttendanceStatuses""#;
    pub const PK: &'static str = r#"AttendanceStatusId::TEXT=$1"#;
    pub const COLUMNS: &'static str = r#""AttendanceStatusId","FullName","Abbreviation","StatusId""#;
    pub const COLUMNS_UPDATE: &'static str = r#""AttendanceStatusId"=$1,"FullName"=$2,"Abbreviation"=$3,"StatusId"=$4 WHERE "AttendanceStatusId"=$1"#;

    pub fn get_id(&self) -> Uuid {
        self.attendance_status_id.clone()
    }

    pub fn new(attendance_status_id: Uuid,full_name: String,abbreviation: String,status_id: Uuid) -> Self {
        Self {
            attendance_status_id,
full_name,
abbreviation,
status_id

        }
    }
}

impl PartialEq for AttendanceStatuses {
    fn eq(&self, other: &Self) -> bool {
        self.attendance_status_id == other.attendance_status_id
    }
}

impl Model for AttendanceStatuses {
    fn from_row(row: &PgRow) -> AttendanceStatuses
    where
        Self: Sized,
    {
        let attendance_status_id = row.get("AttendanceStatusId");
let full_name = row.get("FullName");
let abbreviation = row.get("Abbreviation");
let status_id = row.get("StatusId");


        Self {
            attendance_status_id,
full_name,
abbreviation,
status_id

        }
    }
}
