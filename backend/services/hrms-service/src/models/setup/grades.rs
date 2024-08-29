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
pub struct Grades {
    pub grade_id: Uuid,
pub abbreviation: String,
pub full_name: String,
pub status_id: Uuid

}

impl Grades {
    pub const TABLE: &'static str = r#""Setup"."Grades""#;
    pub const PK: &'static str = r#"GradeId::TEXT=$1"#;
    pub const COLUMNS: &'static str = r#""GradeId","Abbreviation","FullName","StatusId""#;
    pub const COLUMNS_UPDATE: &'static str = r#""GradeId"=$1,"Abbreviation"=$2,"FullName"=$3,"StatusId"=$4 WHERE "GradeId"=$1"#;

    pub fn get_id(&self) -> Uuid {
        self.grade_id.clone()
    }

    pub fn new(grade_id: Uuid,abbreviation: String,full_name: String,status_id: Uuid) -> Self {
        Self {
            grade_id,
abbreviation,
full_name,
status_id

        }
    }
}

impl PartialEq for Grades {
    fn eq(&self, other: &Self) -> bool {
        self.grade_id == other.grade_id
    }
}

impl Model for Grades {
    fn from_row(row: &PgRow) -> Grades
    where
        Self: Sized,
    {
        let grade_id = row.get("GradeId");
let abbreviation = row.get("Abbreviation");
let full_name = row.get("FullName");
let status_id = row.get("StatusId");


        Self {
            grade_id,
abbreviation,
full_name,
status_id

        }
    }
}
