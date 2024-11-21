use c3k_common::interfaces::irepository::Model;
use serde::{Deserialize, Serialize};
pub use sqlx::{
    pool::PoolConnection,
    postgres::{PgArguments, PgPoolOptions, PgRow},
    types::chrono::{DateTime, Utc},
    Arguments, Error, PgPool, Postgres, Row,
};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Status {
    pub status_id: Uuid,
    pub abbreviation: String,
    pub full_name: String,
    pub is_active: bool,
}

impl Status {
    pub const TABLE: &'static str = r#""Setup"."Status""#;
    pub const PK: &'static str = "StatusId";
    pub const COLUMNS_ARRAY: [&'static str; 4] =
        ["StatusId", "Abbreviation", "FullName", "IsActive"];

    pub fn get_id(&self) -> Uuid {
        self.status_id.clone()
    }

    pub fn get_args(&self) -> PgArguments {
        let mut args = PgArguments::default();
        let _ = args.add(self.status_id.clone());
        let _ = args.add(self.abbreviation.clone());
        let _ = args.add(self.full_name.clone());
        let _ = args.add(self.is_active.clone());

        args
    }

    pub fn new(status_id: Uuid, abbreviation: String, full_name: String, is_active: bool) -> Self {
        Self {
            status_id,
            abbreviation,
            full_name,
            is_active,
        }
    }
}

impl PartialEq for Status {
    fn eq(&self, other: &Self) -> bool {
        self.status_id == other.status_id
    }
}

impl Model for Status {
    fn from_row(row: &PgRow) -> Status
    where
        Self: Sized,
    {
        let status_id = row.get("StatusId");
        let abbreviation = row.get("Abbreviation");
        let full_name = row.get("FullName");
        let is_active = row.get("IsActive");

        Self {
            status_id,
            abbreviation,
            full_name,
            is_active,
        }
    }
}
