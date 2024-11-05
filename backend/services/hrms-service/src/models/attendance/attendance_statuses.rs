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
pub struct AttendanceStatuses {
    pub attendance_status_id: Uuid,
pub full_name: String,
pub abbreviation: String,
pub status_id: Uuid

}

impl AttendanceStatuses {
    pub const TABLE: &'static str = r#""Attendance"."AttendanceStatuses""#;
    pub const PK: &'static str = "AttendanceStatusId";
    pub const COLUMNS_ARRAY: [&'static str; 4] = ["AttendanceStatusId","FullName","Abbreviation","StatusId"];

    pub fn get_id(&self) -> Uuid {
        self.attendance_status_id.clone()
    }

    pub fn get_args(&self) -> PgArguments {
        let mut args = PgArguments::default();
        let _ = args.add(self.attendance_status_id.clone());
let _ = args.add(self.full_name.clone());
let _ = args.add(self.abbreviation.clone());
let _ = args.add(self.status_id.clone());

        args
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
