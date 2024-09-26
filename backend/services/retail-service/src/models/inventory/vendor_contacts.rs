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
pub struct VendorContacts {
    pub vendor_contact_id: Uuid,
pub contact_type_id: Uuid,
pub vendor_id: Uuid,
pub contact_value: String,
pub status_id: Uuid

}

impl VendorContacts {
    pub const TABLE: &'static str = r#""Inventory"."VendorContacts""#;
    pub const PK: &'static str = r#"VendorContactId::TEXT=$1"#;
    pub const COLUMNS: &'static str = r#""VendorContactId","ContactTypeId","VendorId","ContactValue","StatusId""#;
    pub const COLUMNS_UPDATE: &'static str = r#""VendorContactId"=$1,"ContactTypeId"=$2,"VendorId"=$3,"ContactValue"=$4,"StatusId"=$5 WHERE "VendorContactId"=$1"#;

    pub fn get_id(&self) -> Uuid {
        self.vendor_contact_id.clone()
    }

    pub fn new(vendor_contact_id: Uuid,contact_type_id: Uuid,vendor_id: Uuid,contact_value: String,status_id: Uuid) -> Self {
        Self {
            vendor_contact_id,
contact_type_id,
vendor_id,
contact_value,
status_id

        }
    }
}

impl PartialEq for VendorContacts {
    fn eq(&self, other: &Self) -> bool {
        self.vendor_contact_id == other.vendor_contact_id
    }
}

impl Model for VendorContacts {
    fn from_row(row: &PgRow) -> VendorContacts
    where
        Self: Sized,
    {
        let vendor_contact_id = row.get("VendorContactId");
let contact_type_id = row.get("ContactTypeId");
let vendor_id = row.get("VendorId");
let contact_value = row.get("ContactValue");
let status_id = row.get("StatusId");


        Self {
            vendor_contact_id,
contact_type_id,
vendor_id,
contact_value,
status_id

        }
    }
}
