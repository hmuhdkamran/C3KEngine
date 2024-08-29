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
pub struct Shifts {
    pub shift_id: Uuid,
pub full_name: String,
pub start_time: DateTime<Utc>,
pub end_time: DateTime<Utc>,
pub status_id: Uuid,
pub abbreviation: String

}

impl Shifts {
    pub const TABLE: &'static str = r#""Attendance"."Shifts""#;
    pub const PK: &'static str = r#"ShiftId::TEXT=$1"#;
    pub const COLUMNS: &'static str = r#""ShiftId","FullName","StartTime","EndTime","StatusId","Abbreviation""#;
    pub const COLUMNS_UPDATE: &'static str = r#""ShiftId"=$1,"FullName"=$2,"StartTime"=$3,"EndTime"=$4,"StatusId"=$5,"Abbreviation"=$6 WHERE "ShiftId"=$1"#;

    pub fn get_id(&self) -> Uuid {
        self.shift_id.clone()
    }

    pub fn new(shift_id: Uuid,full_name: String,start_time: DateTime<Utc>,end_time: DateTime<Utc>,status_id: Uuid,abbreviation: String) -> Self {
        Self {
            shift_id,
full_name,
start_time,
end_time,
status_id,
abbreviation

        }
    }
}

impl PartialEq for Shifts {
    fn eq(&self, other: &Self) -> bool {
        self.shift_id == other.shift_id
    }
}

impl Model for Shifts {
    fn from_row(row: &PgRow) -> Shifts
    where
        Self: Sized,
    {
        let shift_id = row.get("ShiftId");
let full_name = row.get("FullName");
let start_time = row.get("StartTime");
let end_time = row.get("EndTime");
let status_id = row.get("StatusId");
let abbreviation = row.get("Abbreviation");


        Self {
            shift_id,
full_name,
start_time,
end_time,
status_id,
abbreviation

        }
    }
}
