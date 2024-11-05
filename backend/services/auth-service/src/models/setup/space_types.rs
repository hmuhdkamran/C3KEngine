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
pub struct SpaceTypes {
    pub space_type_id: Uuid,
pub abbreviation: String,
pub full_name: String,
pub status_id: Uuid

}

impl SpaceTypes {
    pub const TABLE: &'static str = r#""Setup"."SpaceTypes""#;
    pub const PK: &'static str = "SpaceTypeId";
    pub const COLUMNS_ARRAY: [&'static str; 4] = ["SpaceTypeId","Abbreviation","FullName","StatusId"];

    pub fn get_id(&self) -> Uuid {
        self.space_type_id.clone()
    }

    pub fn get_args(&self) -> PgArguments {
        let mut args = PgArguments::default();
        let _ = args.add(self.space_type_id.clone());
let _ = args.add(self.abbreviation.clone());
let _ = args.add(self.full_name.clone());
let _ = args.add(self.status_id.clone());

        args
    }

    pub fn new(space_type_id: Uuid,abbreviation: String,full_name: String,status_id: Uuid) -> Self {
        Self {
            space_type_id,
abbreviation,
full_name,
status_id

        }
    }
}

impl PartialEq for SpaceTypes {
    fn eq(&self, other: &Self) -> bool {
        self.space_type_id == other.space_type_id
    }
}

impl Model for SpaceTypes {
    fn from_row(row: &PgRow) -> SpaceTypes
    where
        Self: Sized,
    {
        let space_type_id = row.get("SpaceTypeId");
let abbreviation = row.get("Abbreviation");
let full_name = row.get("FullName");
let status_id = row.get("StatusId");


        Self {
            space_type_id,
abbreviation,
full_name,
status_id

        }
    }
}
