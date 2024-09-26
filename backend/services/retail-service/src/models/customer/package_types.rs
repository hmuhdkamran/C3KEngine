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
pub struct PackageTypes {
    pub package_type_id: Uuid,
pub abbreviation: String,
pub full_name: String,
pub status_id: Uuid

}

impl PackageTypes {
    pub const TABLE: &'static str = r#""Customer"."PackageTypes""#;
    pub const PK: &'static str = r#"PackageTypeId::TEXT=$1"#;
    pub const COLUMNS: &'static str = r#""PackageTypeId","Abbreviation","FullName","StatusId""#;
    pub const COLUMNS_UPDATE: &'static str = r#""PackageTypeId"=$1,"Abbreviation"=$2,"FullName"=$3,"StatusId"=$4 WHERE "PackageTypeId"=$1"#;

    pub fn get_id(&self) -> Uuid {
        self.package_type_id.clone()
    }

    pub fn new(package_type_id: Uuid,abbreviation: String,full_name: String,status_id: Uuid) -> Self {
        Self {
            package_type_id,
abbreviation,
full_name,
status_id

        }
    }
}

impl PartialEq for PackageTypes {
    fn eq(&self, other: &Self) -> bool {
        self.package_type_id == other.package_type_id
    }
}

impl Model for PackageTypes {
    fn from_row(row: &PgRow) -> PackageTypes
    where
        Self: Sized,
    {
        let package_type_id = row.get("PackageTypeId");
let abbreviation = row.get("Abbreviation");
let full_name = row.get("FullName");
let status_id = row.get("StatusId");


        Self {
            package_type_id,
abbreviation,
full_name,
status_id

        }
    }
}
