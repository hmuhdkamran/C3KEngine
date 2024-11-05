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
pub struct SpouseTypes {
    pub spouse_type_id: Uuid,
pub abbreviation: String,
pub full_name: String,
pub status_id: Uuid

}

impl SpouseTypes {
    pub const TABLE: &'static str = r#""Setup"."SpouseTypes""#;
    pub const PK: &'static str = "SpouseTypeId";
    pub const COLUMNS_ARRAY: [&'static str; 4] = ["SpouseTypeId","Abbreviation","FullName","StatusId"];

    pub fn get_id(&self) -> Uuid {
        self.spouse_type_id.clone()
    }

    pub fn get_args(&self) -> PgArguments {
        let mut args = PgArguments::default();
        let _ = args.add(self.spouse_type_id.clone());
let _ = args.add(self.abbreviation.clone());
let _ = args.add(self.full_name.clone());
let _ = args.add(self.status_id.clone());

        args
    }

    pub fn new(spouse_type_id: Uuid,abbreviation: String,full_name: String,status_id: Uuid) -> Self {
        Self {
            spouse_type_id,
abbreviation,
full_name,
status_id

        }
    }
}

impl PartialEq for SpouseTypes {
    fn eq(&self, other: &Self) -> bool {
        self.spouse_type_id == other.spouse_type_id
    }
}

impl Model for SpouseTypes {
    fn from_row(row: &PgRow) -> SpouseTypes
    where
        Self: Sized,
    {
        let spouse_type_id = row.get("SpouseTypeId");
let abbreviation = row.get("Abbreviation");
let full_name = row.get("FullName");
let status_id = row.get("StatusId");


        Self {
            spouse_type_id,
abbreviation,
full_name,
status_id

        }
    }
}
