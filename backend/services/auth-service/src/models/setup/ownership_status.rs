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
pub struct OwnershipStatus {
    pub ownership_status_id: Uuid,
pub abbreviation: String,
pub full_name: String,
pub status_id: Uuid

}

impl OwnershipStatus {
    pub const TABLE: &'static str = r#""Setup"."OwnershipStatus""#;
    pub const PK: &'static str = "OwnershipStatusId";
    pub const COLUMNS_ARRAY: [&'static str; 4] = ["OwnershipStatusId","Abbreviation","FullName","StatusId"];

    pub fn get_id(&self) -> Uuid {
        self.ownership_status_id.clone()
    }

    pub fn get_args(&self) -> PgArguments {
        let mut args = PgArguments::default();
        let _ = args.add(self.ownership_status_id.clone());
let _ = args.add(self.abbreviation.clone());
let _ = args.add(self.full_name.clone());
let _ = args.add(self.status_id.clone());

        args
    }

    pub fn new(ownership_status_id: Uuid,abbreviation: String,full_name: String,status_id: Uuid) -> Self {
        Self {
            ownership_status_id,
abbreviation,
full_name,
status_id

        }
    }
}

impl PartialEq for OwnershipStatus {
    fn eq(&self, other: &Self) -> bool {
        self.ownership_status_id == other.ownership_status_id
    }
}

impl Model for OwnershipStatus {
    fn from_row(row: &PgRow) -> OwnershipStatus
    where
        Self: Sized,
    {
        let ownership_status_id = row.get("OwnershipStatusId");
let abbreviation = row.get("Abbreviation");
let full_name = row.get("FullName");
let status_id = row.get("StatusId");


        Self {
            ownership_status_id,
abbreviation,
full_name,
status_id

        }
    }
}
