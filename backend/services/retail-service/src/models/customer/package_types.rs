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
pub struct PackageTypes {
    pub package_type_id: Uuid,
pub abbreviation: String,
pub full_name: String,
pub status_id: Uuid

}

impl PackageTypes {
    pub const TABLE: &'static str = r#""Customer"."PackageTypes""#;
    pub const PK: &'static str = "PackageTypeId";
    pub const COLUMNS_ARRAY: [&'static str; 4] = ["PackageTypeId","Abbreviation","FullName","StatusId"];

    pub fn get_id(&self) -> Uuid {
        self.package_type_id.clone()
    }

    pub fn get_args(&self) -> PgArguments {
        let mut args = PgArguments::default();
        let _ = args.add(self.package_type_id.clone());
let _ = args.add(self.abbreviation.clone());
let _ = args.add(self.full_name.clone());
let _ = args.add(self.status_id.clone());

        args
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
