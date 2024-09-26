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
pub struct CustomerTypes {
    pub customer_type_id: Uuid,
pub abbreviation: String,
pub full_name: String,
pub status_id: Uuid

}

impl CustomerTypes {
    pub const TABLE: &'static str = r#""Customer"."CustomerTypes""#;
    pub const PK: &'static str = r#"CustomerTypeId::TEXT=$1"#;
    pub const COLUMNS: &'static str = r#""CustomerTypeId","Abbreviation","FullName","StatusId""#;
    pub const COLUMNS_UPDATE: &'static str = r#""CustomerTypeId"=$1,"Abbreviation"=$2,"FullName"=$3,"StatusId"=$4 WHERE "CustomerTypeId"=$1"#;

    pub fn get_id(&self) -> Uuid {
        self.customer_type_id.clone()
    }

    pub fn new(customer_type_id: Uuid,abbreviation: String,full_name: String,status_id: Uuid) -> Self {
        Self {
            customer_type_id,
abbreviation,
full_name,
status_id

        }
    }
}

impl PartialEq for CustomerTypes {
    fn eq(&self, other: &Self) -> bool {
        self.customer_type_id == other.customer_type_id
    }
}

impl Model for CustomerTypes {
    fn from_row(row: &PgRow) -> CustomerTypes
    where
        Self: Sized,
    {
        let customer_type_id = row.get("CustomerTypeId");
let abbreviation = row.get("Abbreviation");
let full_name = row.get("FullName");
let status_id = row.get("StatusId");


        Self {
            customer_type_id,
abbreviation,
full_name,
status_id

        }
    }
}
