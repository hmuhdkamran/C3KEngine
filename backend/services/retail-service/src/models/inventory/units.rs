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
pub struct Units {
    pub unit_id: Uuid,
pub abbreviation: String,
pub full_name: String,
pub status_id: Uuid

}

impl Units {
    pub const TABLE: &'static str = r#""Inventory"."Units""#;
    pub const PK: &'static str = r#"UnitId::TEXT=$1"#;
    pub const COLUMNS: &'static str = r#""UnitId","Abbreviation","FullName","StatusId""#;
    pub const COLUMNS_UPDATE: &'static str = r#""UnitId"=$1,"Abbreviation"=$2,"FullName"=$3,"StatusId"=$4 WHERE "UnitId"=$1"#;

    pub fn get_id(&self) -> Uuid {
        self.unit_id.clone()
    }

    pub fn new(unit_id: Uuid,abbreviation: String,full_name: String,status_id: Uuid) -> Self {
        Self {
            unit_id,
abbreviation,
full_name,
status_id

        }
    }
}

impl PartialEq for Units {
    fn eq(&self, other: &Self) -> bool {
        self.unit_id == other.unit_id
    }
}

impl Model for Units {
    fn from_row(row: &PgRow) -> Units
    where
        Self: Sized,
    {
        let unit_id = row.get("UnitId");
let abbreviation = row.get("Abbreviation");
let full_name = row.get("FullName");
let status_id = row.get("StatusId");


        Self {
            unit_id,
abbreviation,
full_name,
status_id

        }
    }
}
