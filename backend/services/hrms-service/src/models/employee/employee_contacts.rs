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
pub struct EmployeeContacts {
    pub employee_contacts_id: Uuid,
pub employee_id: Uuid,
pub contact_type_id: Uuid,
pub contact_value: String,
pub is_primary: bool,
pub status_id: Uuid

}

impl EmployeeContacts {
    pub const TABLE: &'static str = r#""Employee"."EmployeeContacts""#;
    pub const PK: &'static str = r#"EmployeeContactsId::TEXT=$1"#;
    pub const COLUMNS: &'static str = r#""EmployeeContactsId","EmployeeId","ContactTypeId","ContactValue","IsPrimary","StatusId""#;
    pub const COLUMNS_UPDATE: &'static str = r#""EmployeeContactsId"=$1,"EmployeeId"=$2,"ContactTypeId"=$3,"ContactValue"=$4,"IsPrimary"=$5,"StatusId"=$6 WHERE "EmployeeContactsId"=$1"#;

    pub fn get_id(&self) -> Uuid {
        self.employee_contacts_id.clone()
    }

    pub fn new(employee_contacts_id: Uuid,employee_id: Uuid,contact_type_id: Uuid,contact_value: String,is_primary: bool,status_id: Uuid) -> Self {
        Self {
            employee_contacts_id,
employee_id,
contact_type_id,
contact_value,
is_primary,
status_id

        }
    }
}

impl PartialEq for EmployeeContacts {
    fn eq(&self, other: &Self) -> bool {
        self.employee_contacts_id == other.employee_contacts_id
    }
}

impl Model for EmployeeContacts {
    fn from_row(row: &PgRow) -> EmployeeContacts
    where
        Self: Sized,
    {
        let employee_contacts_id = row.get("EmployeeContactsId");
let employee_id = row.get("EmployeeId");
let contact_type_id = row.get("ContactTypeId");
let contact_value = row.get("ContactValue");
let is_primary = row.get("IsPrimary");
let status_id = row.get("StatusId");


        Self {
            employee_contacts_id,
employee_id,
contact_type_id,
contact_value,
is_primary,
status_id

        }
    }
}
