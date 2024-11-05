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
pub struct Categories {
    pub category_id: Uuid,
pub abbreviation: String,
pub full_name: String,
pub status_id: Uuid

}

impl Categories {
    pub const TABLE: &'static str = r#""Setup"."Categories""#;
    pub const PK: &'static str = "CategoryId";
    pub const COLUMNS_ARRAY: [&'static str; 4] = ["CategoryId","Abbreviation","FullName","StatusId"];

    pub fn get_id(&self) -> Uuid {
        self.category_id.clone()
    }

    pub fn get_args(&self) -> PgArguments {
        let mut args = PgArguments::default();
        let _ = args.add(self.category_id.clone());
let _ = args.add(self.abbreviation.clone());
let _ = args.add(self.full_name.clone());
let _ = args.add(self.status_id.clone());

        args
    }

    pub fn new(category_id: Uuid,abbreviation: String,full_name: String,status_id: Uuid) -> Self {
        Self {
            category_id,
abbreviation,
full_name,
status_id

        }
    }
}

impl PartialEq for Categories {
    fn eq(&self, other: &Self) -> bool {
        self.category_id == other.category_id
    }
}

impl Model for Categories {
    fn from_row(row: &PgRow) -> Categories
    where
        Self: Sized,
    {
        let category_id = row.get("CategoryId");
let abbreviation = row.get("Abbreviation");
let full_name = row.get("FullName");
let status_id = row.get("StatusId");


        Self {
            category_id,
abbreviation,
full_name,
status_id

        }
    }
}
