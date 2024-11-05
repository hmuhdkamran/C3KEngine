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
pub struct Designations {
    pub designation_id: Uuid,
pub full_name: String,
pub status_id: Uuid,
pub abbreviation: String

}

impl Designations {
    pub const TABLE: &'static str = r#""Setup"."Designations""#;
    pub const PK: &'static str = "DesignationId";
    pub const COLUMNS_ARRAY: [&'static str; 4] = ["DesignationId","FullName","StatusId","Abbreviation"];

    pub fn get_id(&self) -> Uuid {
        self.designation_id.clone()
    }

    pub fn get_args(&self) -> PgArguments {
        let mut args = PgArguments::default();
        let _ = args.add(self.designation_id.clone());
let _ = args.add(self.full_name.clone());
let _ = args.add(self.status_id.clone());
let _ = args.add(self.abbreviation.clone());

        args
    }

    pub fn new(designation_id: Uuid,full_name: String,status_id: Uuid,abbreviation: String) -> Self {
        Self {
            designation_id,
full_name,
status_id,
abbreviation

        }
    }
}

impl PartialEq for Designations {
    fn eq(&self, other: &Self) -> bool {
        self.designation_id == other.designation_id
    }
}

impl Model for Designations {
    fn from_row(row: &PgRow) -> Designations
    where
        Self: Sized,
    {
        let designation_id = row.get("DesignationId");
let full_name = row.get("FullName");
let status_id = row.get("StatusId");
let abbreviation = row.get("Abbreviation");


        Self {
            designation_id,
full_name,
status_id,
abbreviation

        }
    }
}
