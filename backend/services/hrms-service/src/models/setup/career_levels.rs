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
pub struct CareerLevels {
    pub career_level_id: Uuid,
pub full_name: String,
pub status_id: Uuid,
pub abbreviation: String

}

impl CareerLevels {
    pub const TABLE: &'static str = r#""Setup"."CareerLevels""#;
    pub const PK: &'static str = "CareerLevelId";
    pub const COLUMNS_ARRAY: [&'static str; 4] = ["CareerLevelId","FullName","StatusId","Abbreviation"];

    pub fn get_id(&self) -> Uuid {
        self.career_level_id.clone()
    }

    pub fn get_args(&self) -> PgArguments {
        let mut args = PgArguments::default();
        let _ = args.add(self.career_level_id.clone());
let _ = args.add(self.full_name.clone());
let _ = args.add(self.status_id.clone());
let _ = args.add(self.abbreviation.clone());

        args
    }

    pub fn new(career_level_id: Uuid,full_name: String,status_id: Uuid,abbreviation: String) -> Self {
        Self {
            career_level_id,
full_name,
status_id,
abbreviation

        }
    }
}

impl PartialEq for CareerLevels {
    fn eq(&self, other: &Self) -> bool {
        self.career_level_id == other.career_level_id
    }
}

impl Model for CareerLevels {
    fn from_row(row: &PgRow) -> CareerLevels
    where
        Self: Sized,
    {
        let career_level_id = row.get("CareerLevelId");
let full_name = row.get("FullName");
let status_id = row.get("StatusId");
let abbreviation = row.get("Abbreviation");


        Self {
            career_level_id,
full_name,
status_id,
abbreviation

        }
    }
}
