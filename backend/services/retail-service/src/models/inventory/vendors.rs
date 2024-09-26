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
pub struct Vendors {
    pub vendor_id: Uuid,
pub code: String,
pub full_name: String,
pub branch_id: Uuid,
pub picture: String,
pub contact_person: String,
pub status_id: Uuid

}

impl Vendors {
    pub const TABLE: &'static str = r#""Inventory"."Vendors""#;
    pub const PK: &'static str = r#"VendorId::TEXT=$1"#;
    pub const COLUMNS: &'static str = r#""VendorId","Code","FullName","BranchId","Picture","ContactPerson","StatusId""#;
    pub const COLUMNS_UPDATE: &'static str = r#""VendorId"=$1,"Code"=$2,"FullName"=$3,"BranchId"=$4,"Picture"=$5,"ContactPerson"=$6,"StatusId"=$7 WHERE "VendorId"=$1"#;

    pub fn get_id(&self) -> Uuid {
        self.vendor_id.clone()
    }

    pub fn new(vendor_id: Uuid,code: String,full_name: String,branch_id: Uuid,picture: String,contact_person: String,status_id: Uuid) -> Self {
        Self {
            vendor_id,
code,
full_name,
branch_id,
picture,
contact_person,
status_id

        }
    }
}

impl PartialEq for Vendors {
    fn eq(&self, other: &Self) -> bool {
        self.vendor_id == other.vendor_id
    }
}

impl Model for Vendors {
    fn from_row(row: &PgRow) -> Vendors
    where
        Self: Sized,
    {
        let vendor_id = row.get("VendorId");
let code = row.get("Code");
let full_name = row.get("FullName");
let branch_id = row.get("BranchId");
let picture = row.get("Picture");
let contact_person = row.get("ContactPerson");
let status_id = row.get("StatusId");


        Self {
            vendor_id,
code,
full_name,
branch_id,
picture,
contact_person,
status_id

        }
    }
}
