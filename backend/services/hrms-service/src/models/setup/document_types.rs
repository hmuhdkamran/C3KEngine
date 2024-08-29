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
pub struct DocumentTypes {
    pub document_type_id: Uuid,
pub full_name: String,
pub status_id: Uuid,
pub abbreviation: String

}

impl DocumentTypes {
    pub const TABLE: &'static str = r#""Setup"."DocumentTypes""#;
    pub const PK: &'static str = r#"DocumentTypeId::TEXT=$1"#;
    pub const COLUMNS: &'static str = r#""DocumentTypeId","FullName","StatusId","Abbreviation""#;
    pub const COLUMNS_UPDATE: &'static str = r#""DocumentTypeId"=$1,"FullName"=$2,"StatusId"=$3,"Abbreviation"=$4 WHERE "DocumentTypeId"=$1"#;

    pub fn get_id(&self) -> Uuid {
        self.document_type_id.clone()
    }

    pub fn new(document_type_id: Uuid,full_name: String,status_id: Uuid,abbreviation: String) -> Self {
        Self {
            document_type_id,
full_name,
status_id,
abbreviation

        }
    }
}

impl PartialEq for DocumentTypes {
    fn eq(&self, other: &Self) -> bool {
        self.document_type_id == other.document_type_id
    }
}

impl Model for DocumentTypes {
    fn from_row(row: &PgRow) -> DocumentTypes
    where
        Self: Sized,
    {
        let document_type_id = row.get("DocumentTypeId");
let full_name = row.get("FullName");
let status_id = row.get("StatusId");
let abbreviation = row.get("Abbreviation");


        Self {
            document_type_id,
full_name,
status_id,
abbreviation

        }
    }
}
