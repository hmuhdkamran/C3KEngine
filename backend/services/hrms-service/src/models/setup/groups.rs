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
pub struct Groups {
    pub group_id: Uuid,
pub abbreviation: String,
pub full_name: String,
pub status_id: Uuid

}

impl Groups {
    pub const TABLE: &'static str = r#""Setup"."Groups""#;
    pub const PK: &'static str = r#"GroupId::TEXT=$1"#;
    pub const COLUMNS: &'static str = r#""GroupId","Abbreviation","FullName","StatusId""#;
    pub const COLUMNS_UPDATE: &'static str = r#""GroupId"=$1,"Abbreviation"=$2,"FullName"=$3,"StatusId"=$4 WHERE "GroupId"=$1"#;

    pub fn get_id(&self) -> Uuid {
        self.group_id.clone()
    }

    pub fn new(group_id: Uuid,abbreviation: String,full_name: String,status_id: Uuid) -> Self {
        Self {
            group_id,
abbreviation,
full_name,
status_id

        }
    }
}

impl PartialEq for Groups {
    fn eq(&self, other: &Self) -> bool {
        self.group_id == other.group_id
    }
}

impl Model for Groups {
    fn from_row(row: &PgRow) -> Groups
    where
        Self: Sized,
    {
        let group_id = row.get("GroupId");
let abbreviation = row.get("Abbreviation");
let full_name = row.get("FullName");
let status_id = row.get("StatusId");


        Self {
            group_id,
abbreviation,
full_name,
status_id

        }
    }
}
