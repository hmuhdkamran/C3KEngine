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
pub struct CustomerContacts {
    pub customer_contact_id: Uuid,
pub contact_type_id: Uuid,
pub contact_value: String,
pub status_id: Uuid,
pub customer_id: Uuid

}

impl CustomerContacts {
    pub const TABLE: &'static str = r#""Customer"."CustomerContacts""#;
    pub const PK: &'static str = r#"CustomerContactId::TEXT=$1"#;
    pub const COLUMNS: &'static str = r#""CustomerContactId","ContactTypeId","ContactValue","StatusId","CustomerId""#;
    pub const COLUMNS_UPDATE: &'static str = r#""CustomerContactId"=$1,"ContactTypeId"=$2,"ContactValue"=$3,"StatusId"=$4,"CustomerId"=$5 WHERE "CustomerContactId"=$1"#;

    pub fn get_id(&self) -> Uuid {
        self.customer_contact_id.clone()
    }

    pub fn new(customer_contact_id: Uuid,contact_type_id: Uuid,contact_value: String,status_id: Uuid,customer_id: Uuid) -> Self {
        Self {
            customer_contact_id,
contact_type_id,
contact_value,
status_id,
customer_id

        }
    }
}

impl PartialEq for CustomerContacts {
    fn eq(&self, other: &Self) -> bool {
        self.customer_contact_id == other.customer_contact_id
    }
}

impl Model for CustomerContacts {
    fn from_row(row: &PgRow) -> CustomerContacts
    where
        Self: Sized,
    {
        let customer_contact_id = row.get("CustomerContactId");
let contact_type_id = row.get("ContactTypeId");
let contact_value = row.get("ContactValue");
let status_id = row.get("StatusId");
let customer_id = row.get("CustomerId");


        Self {
            customer_contact_id,
contact_type_id,
contact_value,
status_id,
customer_id

        }
    }
}
