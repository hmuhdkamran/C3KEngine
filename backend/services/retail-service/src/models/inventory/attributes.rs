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
pub struct Attributes {
    pub attribute_id: Uuid,
pub abbreviation: String,
pub full_name: String,
pub status_id: Uuid

}

impl Attributes {
    pub const TABLE: &'static str = r#""Inventory"."Attributes""#;
    pub const PK: &'static str = "AttributeId";
    pub const COLUMNS_ARRAY: [&'static str; 4] = ["AttributeId","Abbreviation","FullName","StatusId"];

    pub fn get_id(&self) -> Uuid {
        self.attribute_id.clone()
    }

    pub fn get_args(&self) -> PgArguments {
        let mut args = PgArguments::default();
        let _ = args.add(self.attribute_id.clone());
let _ = args.add(self.abbreviation.clone());
let _ = args.add(self.full_name.clone());
let _ = args.add(self.status_id.clone());

        args
    }

    pub fn new(attribute_id: Uuid,abbreviation: String,full_name: String,status_id: Uuid) -> Self {
        Self {
            attribute_id,
abbreviation,
full_name,
status_id

        }
    }
}

impl PartialEq for Attributes {
    fn eq(&self, other: &Self) -> bool {
        self.attribute_id == other.attribute_id
    }
}

impl Model for Attributes {
    fn from_row(row: &PgRow) -> Attributes
    where
        Self: Sized,
    {
        let attribute_id = row.get("AttributeId");
let abbreviation = row.get("Abbreviation");
let full_name = row.get("FullName");
let status_id = row.get("StatusId");


        Self {
            attribute_id,
abbreviation,
full_name,
status_id

        }
    }
}
