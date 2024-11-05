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
pub struct Religions {
    pub religion_id: Uuid,
pub full_name: String,
pub status_id: Uuid,
pub abbreviation: String

}

impl Religions {
    pub const TABLE: &'static str = r#""Setup"."Religions""#;
    pub const PK: &'static str = "ReligionId";
    pub const COLUMNS_ARRAY: [&'static str; 4] = ["ReligionId","FullName","StatusId","Abbreviation"];

    pub fn get_id(&self) -> Uuid {
        self.religion_id.clone()
    }

    pub fn get_args(&self) -> PgArguments {
        let mut args = PgArguments::default();
        let _ = args.add(self.religion_id.clone());
let _ = args.add(self.full_name.clone());
let _ = args.add(self.status_id.clone());
let _ = args.add(self.abbreviation.clone());

        args
    }

    pub fn new(religion_id: Uuid,full_name: String,status_id: Uuid,abbreviation: String) -> Self {
        Self {
            religion_id,
full_name,
status_id,
abbreviation

        }
    }
}

impl PartialEq for Religions {
    fn eq(&self, other: &Self) -> bool {
        self.religion_id == other.religion_id
    }
}

impl Model for Religions {
    fn from_row(row: &PgRow) -> Religions
    where
        Self: Sized,
    {
        let religion_id = row.get("ReligionId");
let full_name = row.get("FullName");
let status_id = row.get("StatusId");
let abbreviation = row.get("Abbreviation");


        Self {
            religion_id,
full_name,
status_id,
abbreviation

        }
    }
}
