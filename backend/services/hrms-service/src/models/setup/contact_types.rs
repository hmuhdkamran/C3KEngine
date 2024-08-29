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
pub struct ContactTypes {
    pub contact_type_id: Uuid,
pub full_name: String,
pub status_id: Uuid,
pub abbreviation: String

}

impl ContactTypes {
    pub const TABLE: &'static str = r#""Setup"."ContactTypes""#;
    pub const PK: &'static str = r#"ContactTypeId::TEXT=$1"#;
    pub const COLUMNS: &'static str = r#""ContactTypeId","FullName","StatusId","Abbreviation""#;
    pub const COLUMNS_UPDATE: &'static str = r#""ContactTypeId"=$1,"FullName"=$2,"StatusId"=$3,"Abbreviation"=$4 WHERE "ContactTypeId"=$1"#;

    pub fn get_id(&self) -> Uuid {
        self.contact_type_id.clone()
    }

    pub fn new(contact_type_id: Uuid,full_name: String,status_id: Uuid,abbreviation: String) -> Self {
        Self {
            contact_type_id,
full_name,
status_id,
abbreviation

        }
    }
}

impl PartialEq for ContactTypes {
    fn eq(&self, other: &Self) -> bool {
        self.contact_type_id == other.contact_type_id
    }
}

impl Model for ContactTypes {
    fn from_row(row: &PgRow) -> ContactTypes
    where
        Self: Sized,
    {
        let contact_type_id = row.get("ContactTypeId");
let full_name = row.get("FullName");
let status_id = row.get("StatusId");
let abbreviation = row.get("Abbreviation");


        Self {
            contact_type_id,
full_name,
status_id,
abbreviation

        }
    }
}
