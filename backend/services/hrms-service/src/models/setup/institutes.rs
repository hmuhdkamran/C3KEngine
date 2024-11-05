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
pub struct Institutes {
    pub institute_id: Uuid,
pub full_name: String,
pub status_id: Uuid,
pub abbreviation: String

}

impl Institutes {
    pub const TABLE: &'static str = r#""Setup"."Institutes""#;
    pub const PK: &'static str = "InstituteId";
    pub const COLUMNS_ARRAY: [&'static str; 4] = ["InstituteId","FullName","StatusId","Abbreviation"];

    pub fn get_id(&self) -> Uuid {
        self.institute_id.clone()
    }

    pub fn get_args(&self) -> PgArguments {
        let mut args = PgArguments::default();
        let _ = args.add(self.institute_id.clone());
let _ = args.add(self.full_name.clone());
let _ = args.add(self.status_id.clone());
let _ = args.add(self.abbreviation.clone());

        args
    }

    pub fn new(institute_id: Uuid,full_name: String,status_id: Uuid,abbreviation: String) -> Self {
        Self {
            institute_id,
full_name,
status_id,
abbreviation

        }
    }
}

impl PartialEq for Institutes {
    fn eq(&self, other: &Self) -> bool {
        self.institute_id == other.institute_id
    }
}

impl Model for Institutes {
    fn from_row(row: &PgRow) -> Institutes
    where
        Self: Sized,
    {
        let institute_id = row.get("InstituteId");
let full_name = row.get("FullName");
let status_id = row.get("StatusId");
let abbreviation = row.get("Abbreviation");


        Self {
            institute_id,
full_name,
status_id,
abbreviation

        }
    }
}
