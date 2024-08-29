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
pub struct JobShifts {
    pub job_shift_id: Uuid,
pub full_name: String,
pub abbreviation: String,
pub status_id: Uuid

}

impl JobShifts {
    pub const TABLE: &'static str = r#""Recruitment"."JobShifts""#;
    pub const PK: &'static str = r#"JobShiftId::TEXT=$1"#;
    pub const COLUMNS: &'static str = r#""JobShiftId","FullName","Abbreviation","StatusId""#;
    pub const COLUMNS_UPDATE: &'static str = r#""JobShiftId"=$1,"FullName"=$2,"Abbreviation"=$3,"StatusId"=$4 WHERE "JobShiftId"=$1"#;

    pub fn get_id(&self) -> Uuid {
        self.job_shift_id.clone()
    }

    pub fn new(job_shift_id: Uuid,full_name: String,abbreviation: String,status_id: Uuid) -> Self {
        Self {
            job_shift_id,
full_name,
abbreviation,
status_id

        }
    }
}

impl PartialEq for JobShifts {
    fn eq(&self, other: &Self) -> bool {
        self.job_shift_id == other.job_shift_id
    }
}

impl Model for JobShifts {
    fn from_row(row: &PgRow) -> JobShifts
    where
        Self: Sized,
    {
        let job_shift_id = row.get("JobShiftId");
let full_name = row.get("FullName");
let abbreviation = row.get("Abbreviation");
let status_id = row.get("StatusId");


        Self {
            job_shift_id,
full_name,
abbreviation,
status_id

        }
    }
}
