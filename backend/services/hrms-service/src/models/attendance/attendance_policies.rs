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
#[serde(rename_all = "PascalCase")]
pub struct AttendancePolicies {
    pub attendance_policie_id: Uuid,
pub full_name: String,
pub description: String,
pub status_id: Uuid

}

impl AttendancePolicies {
    pub const TABLE: &'static str = r#""Attendance"."AttendancePolicies""#;
    pub const PK: &'static str = "AttendancePolicieId";
    pub const COLUMNS_ARRAY: [&'static str; 4] = ["AttendancePolicieId","FullName","Description","StatusId"];

    pub fn get_id(&self) -> Uuid {
        self.attendance_policie_id.clone()
    }

    pub fn get_args(&self) -> PgArguments {
        let mut args = PgArguments::default();
        let _ = args.add(self.attendance_policie_id.clone());
let _ = args.add(self.full_name.clone());
let _ = args.add(self.description.clone());
let _ = args.add(self.status_id.clone());

        args
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
