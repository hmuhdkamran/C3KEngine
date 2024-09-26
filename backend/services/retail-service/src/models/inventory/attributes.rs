use c3k_common::interfaces::irepository::Model;
use serde::{Deserialize, Serialize};
pub use sqlx::{
    pool::PoolConnection,
    postgres::{PgArguments, PgPoolOptions, PgRow},
    types::chrono::{DateTime, Utc},
    Arguments, Error, PgPool, Postgres, Row,
};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Attributes {
    pub attribute_id: Uuid,
    pub abbreviation: String,
    pub full_name: String,
    pub status_id: Uuid,
}

impl Attributes {
    pub const TABLE: &'static str = r#""Inventory"."Attributes""#;
    pub const PK: &'static str = r#"AttributeId::TEXT=$1"#;
    pub const COLUMNS: &'static str =
        r#""AttributeId","AttributeId","Abbreviation","FullName","StatusId""#;
    pub const COLUMNS_UPDATE: &'static str = r#""AttributeId"=$1,"AttributeId"=$1,"Abbreviation"=$2,"FullName"=$3,"StatusId"=$4 WHERE "AttributeId"=$1"#;

    pub fn get_id(&self) -> Uuid {
        self.attribute_id.clone()
    }

    pub fn new(
        attribute_id: Uuid,
        abbreviation: String,
        full_name: String,
        status_id: Uuid,
    ) -> Self {
        Self {
            attribute_id,
            abbreviation,
            full_name,
            status_id,
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
            status_id,
        }
    }
}
