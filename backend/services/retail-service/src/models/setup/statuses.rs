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
pub struct Statuses {
    pub status_id: Uuid,
pub abberviation: String,
pub full_name: String,
pub is_active: bool

}

impl Statuses {
    pub const TABLE: &'static str = r#""Setup"."Statuses""#;
    pub const PK: &'static str = r#"StatusId::TEXT=$1"#;
    pub const COLUMNS: &'static str = r#""StatusId","Abberviation","FullName","IsActive""#;
    pub const COLUMNS_UPDATE: &'static str = r#""StatusId"=$1,"Abberviation"=$2,"FullName"=$3,"IsActive"=$4 WHERE "StatusId"=$1"#;

    pub fn get_id(&self) -> Uuid {
        self.status_id.clone()
    }

    pub fn new(status_id: Uuid,abberviation: String,full_name: String,is_active: bool) -> Self {
        Self {
            status_id,
abberviation,
full_name,
is_active

        }
    }
}

impl PartialEq for Statuses {
    fn eq(&self, other: &Self) -> bool {
        self.status_id == other.status_id
    }
}

impl Model for Statuses {
    fn from_row(row: &PgRow) -> Statuses
    where
        Self: Sized,
    {
        let status_id = row.get("StatusId");
let abberviation = row.get("Abberviation");
let full_name = row.get("FullName");
let is_active = row.get("IsActive");


        Self {
            status_id,
abberviation,
full_name,
is_active

        }
    }
}
