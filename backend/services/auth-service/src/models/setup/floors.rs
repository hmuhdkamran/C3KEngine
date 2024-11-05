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
pub struct Floors {
    pub floor_id: Uuid,
pub abbreviation: String,
pub full_name: String,
pub status_id: Uuid

}

impl Floors {
    pub const TABLE: &'static str = r#""Setup"."Floors""#;
    pub const PK: &'static str = "FloorId";
    pub const COLUMNS_ARRAY: [&'static str; 4] = ["FloorId","Abbreviation","FullName","StatusId"];

    pub fn get_id(&self) -> Uuid {
        self.floor_id.clone()
    }

    pub fn get_args(&self) -> PgArguments {
        let mut args = PgArguments::default();
        let _ = args.add(self.floor_id.clone());
let _ = args.add(self.abbreviation.clone());
let _ = args.add(self.full_name.clone());
let _ = args.add(self.status_id.clone());

        args
    }

    pub fn new(floor_id: Uuid,abbreviation: String,full_name: String,status_id: Uuid) -> Self {
        Self {
            floor_id,
abbreviation,
full_name,
status_id

        }
    }
}

impl PartialEq for Floors {
    fn eq(&self, other: &Self) -> bool {
        self.floor_id == other.floor_id
    }
}

impl Model for Floors {
    fn from_row(row: &PgRow) -> Floors
    where
        Self: Sized,
    {
        let floor_id = row.get("FloorId");
let abbreviation = row.get("Abbreviation");
let full_name = row.get("FullName");
let status_id = row.get("StatusId");


        Self {
            floor_id,
abbreviation,
full_name,
status_id

        }
    }
}
