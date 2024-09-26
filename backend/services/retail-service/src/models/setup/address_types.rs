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
pub abberviation: String,
pub full_name: String,
pub status_id: Uuid

}

impl AddressTypes {
    pub const TABLE: &'static str = r#""Setup"."AddressTypes""#;
    pub const PK: &'static str = r#"AddressTypeId::TEXT=$1"#;
    pub const COLUMNS: &'static str = r#""AddressTypeId","Abberviation","FullName","StatusId""#;
    pub const COLUMNS_UPDATE: &'static str = r#""AddressTypeId"=$1,"Abberviation"=$2,"FullName"=$3,"StatusId"=$4 WHERE "AddressTypeId"=$1"#;

    pub fn get_id(&self) -> Uuid {
        self.address_type_id.clone()
    }

    pub fn new(address_type_id: Uuid,abberviation: String,full_name: String,status_id: Uuid) -> Self {
        Self {
            address_type_id,
abberviation,
full_name,
status_id

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
let abberviation = row.get("Abberviation");
let full_name = row.get("FullName");
let status_id = row.get("StatusId");


        Self {
            address_type_id,
abberviation,
full_name,
status_id

        }
    }
}
