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
pub struct AttendancePolicies {
    pub attendance_policie_id: Uuid,
pub full_name: String,
pub description: String,
pub status_id: Uuid

}

impl AttendancePolicies {
    pub const TABLE: &'static str = r#""Attendance"."AttendancePolicies""#;
    pub const PK: &'static str = r#"AttendancePolicieId::TEXT=$1"#;
    pub const COLUMNS: &'static str = r#""AttendancePolicieId","FullName","Description","StatusId""#;
    pub const COLUMNS_UPDATE: &'static str = r#""AttendancePolicieId"=$1,"FullName"=$2,"Description"=$3,"StatusId"=$4 WHERE "AttendancePolicieId"=$1"#;

    pub fn get_id(&self) -> Uuid {
        self.attendance_policie_id.clone()
    }

    pub fn new(attendance_policie_id: Uuid,full_name: String,description: String,status_id: Uuid) -> Self {
        Self {
            attendance_policie_id,
full_name,
description,
status_id

        }
    }
}

impl PartialEq for AttendancePolicies {
    fn eq(&self, other: &Self) -> bool {
        self.attendance_policie_id == other.attendance_policie_id
    }
}

impl Model for AttendancePolicies {
    fn from_row(row: &PgRow) -> AttendancePolicies
    where
        Self: Sized,
    {
        let attendance_policie_id = row.get("AttendancePolicieId");
let full_name = row.get("FullName");
let description = row.get("Description");
let status_id = row.get("StatusId");


        Self {
            attendance_policie_id,
full_name,
description,
status_id

        }
    }
}
