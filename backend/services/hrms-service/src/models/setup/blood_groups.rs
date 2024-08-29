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
pub struct BloodGroups {
    pub blood_group_id: Uuid,
pub full_name: String,
pub status_id: Uuid,
pub abbreviation: String

}

impl BloodGroups {
    pub const TABLE: &'static str = r#""Setup"."BloodGroups""#;
    pub const PK: &'static str = r#"BloodGroupId::TEXT=$1"#;
    pub const COLUMNS: &'static str = r#""BloodGroupId","FullName","StatusId","Abbreviation""#;
    pub const COLUMNS_UPDATE: &'static str = r#""BloodGroupId"=$1,"FullName"=$2,"StatusId"=$3,"Abbreviation"=$4 WHERE "BloodGroupId"=$1"#;

    pub fn get_id(&self) -> Uuid {
        self.blood_group_id.clone()
    }

    pub fn new(blood_group_id: Uuid,full_name: String,status_id: Uuid,abbreviation: String) -> Self {
        Self {
            blood_group_id,
full_name,
status_id,
abbreviation

        }
    }
}

impl PartialEq for BloodGroups {
    fn eq(&self, other: &Self) -> bool {
        self.blood_group_id == other.blood_group_id
    }
}

impl Model for BloodGroups {
    fn from_row(row: &PgRow) -> BloodGroups
    where
        Self: Sized,
    {
        let blood_group_id = row.get("BloodGroupId");
let full_name = row.get("FullName");
let status_id = row.get("StatusId");
let abbreviation = row.get("Abbreviation");


        Self {
            blood_group_id,
full_name,
status_id,
abbreviation

        }
    }
}
