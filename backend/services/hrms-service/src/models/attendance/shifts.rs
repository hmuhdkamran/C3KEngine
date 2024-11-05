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
    pub const PK: &'static str = "ShiftId";
    pub const COLUMNS_ARRAY: [&'static str; 6] = ["ShiftId","FullName","StartTime","EndTime","StatusId","Abbreviation"];

    pub fn get_id(&self) -> Uuid {
        self.shift_id.clone()
    }

    pub fn get_args(&self) -> PgArguments {
        let mut args = PgArguments::default();
        let _ = args.add(self.shift_id.clone());
let _ = args.add(self.full_name.clone());
let _ = args.add(self.start_time.clone());
let _ = args.add(self.end_time.clone());
let _ = args.add(self.status_id.clone());
let _ = args.add(self.abbreviation.clone());

        args
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
