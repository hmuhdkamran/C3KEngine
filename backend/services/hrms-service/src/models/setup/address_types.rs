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
pub struct AddressTypes {
    pub address_type_id: Uuid,
pub full_name: String,
pub status_id: Uuid,
pub abbreviation: String

}

impl AddressTypes {
    pub const TABLE: &'static str = r#""Setup"."AddressTypes""#;
    pub const PK: &'static str = r#"AddressTypeId::TEXT=$1"#;
    pub const COLUMNS: &'static str = r#""AddressTypeId","FullName","StatusId","Abbreviation""#;
    pub const COLUMNS_UPDATE: &'static str = r#""AddressTypeId"=$1,"FullName"=$2,"StatusId"=$3,"Abbreviation"=$4 WHERE "AddressTypeId"=$1"#;

    pub fn get_id(&self) -> Uuid {
        self.address_type_id.clone()
    }

    pub fn new(address_type_id: Uuid,full_name: String,status_id: Uuid,abbreviation: String) -> Self {
        Self {
            address_type_id,
full_name,
status_id,
abbreviation

        }
    }
}

impl PartialEq for AddressTypes {
    fn eq(&self, other: &Self) -> bool {
        self.address_type_id == other.address_type_id
    }
}

impl Model for AddressTypes {
    fn from_row(row: &PgRow) -> AddressTypes
    where
        Self: Sized,
    {
        let address_type_id = row.get("AddressTypeId");
let full_name = row.get("FullName");
let status_id = row.get("StatusId");
let abbreviation = row.get("Abbreviation");


        Self {
            address_type_id,
full_name,
status_id,
abbreviation

        }
    }
}
