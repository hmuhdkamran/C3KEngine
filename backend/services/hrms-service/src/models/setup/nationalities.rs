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
pub struct Nationalities {
    pub nationality_id: Uuid,
pub full_name: String,
pub status_id: Uuid,
pub abbreviation: String

}

impl Nationalities {
    pub const TABLE: &'static str = r#""Setup"."Nationalities""#;
    pub const PK: &'static str = "NationalityId";
    pub const COLUMNS_ARRAY: [&'static str; 4] = ["NationalityId","FullName","StatusId","Abbreviation"];

    pub fn get_id(&self) -> Uuid {
        self.nationality_id.clone()
    }

    pub fn get_args(&self) -> PgArguments {
        let mut args = PgArguments::default();
        let _ = args.add(self.nationality_id.clone());
let _ = args.add(self.full_name.clone());
let _ = args.add(self.status_id.clone());
let _ = args.add(self.abbreviation.clone());

        args
    }

    pub fn new(nationality_id: Uuid,full_name: String,status_id: Uuid,abbreviation: String) -> Self {
        Self {
            nationality_id,
full_name,
status_id,
abbreviation

        }
    }
}

impl PartialEq for Nationalities {
    fn eq(&self, other: &Self) -> bool {
        self.nationality_id == other.nationality_id
    }
}

impl Model for Nationalities {
    fn from_row(row: &PgRow) -> Nationalities
    where
        Self: Sized,
    {
        let nationality_id = row.get("NationalityId");
let full_name = row.get("FullName");
let status_id = row.get("StatusId");
let abbreviation = row.get("Abbreviation");


        Self {
            nationality_id,
full_name,
status_id,
abbreviation

        }
    }
}
