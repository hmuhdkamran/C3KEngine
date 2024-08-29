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
pub struct SpouseContacts {
    pub spouse_contact_id: Uuid,
pub contact_type_id: Uuid,
pub spouse_id: Uuid,
pub contact_info: String,
pub status_id: Uuid

}

impl SpouseContacts {
    pub const TABLE: &'static str = r#""Employee"."SpouseContacts""#;
    pub const PK: &'static str = r#"SpouseContactId::TEXT=$1"#;
    pub const COLUMNS: &'static str = r#""SpouseContactId","ContactTypeId","SpouseId","ContactInfo","StatusId""#;
    pub const COLUMNS_UPDATE: &'static str = r#""SpouseContactId"=$1,"ContactTypeId"=$2,"SpouseId"=$3,"ContactInfo"=$4,"StatusId"=$5 WHERE "SpouseContactId"=$1"#;

    pub fn get_id(&self) -> Uuid {
        self.spouse_contact_id.clone()
    }

    pub fn new(spouse_contact_id: Uuid,contact_type_id: Uuid,spouse_id: Uuid,contact_info: String,status_id: Uuid) -> Self {
        Self {
            spouse_contact_id,
contact_type_id,
spouse_id,
contact_info,
status_id

        }
    }
}

impl PartialEq for SpouseContacts {
    fn eq(&self, other: &Self) -> bool {
        self.spouse_contact_id == other.spouse_contact_id
    }
}

impl Model for SpouseContacts {
    fn from_row(row: &PgRow) -> SpouseContacts
    where
        Self: Sized,
    {
        let spouse_contact_id = row.get("SpouseContactId");
let contact_type_id = row.get("ContactTypeId");
let spouse_id = row.get("SpouseId");
let contact_info = row.get("ContactInfo");
let status_id = row.get("StatusId");


        Self {
            spouse_contact_id,
contact_type_id,
spouse_id,
contact_info,
status_id

        }
    }
}
