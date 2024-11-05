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
    pub const PK: &'static str = "EmployeeContactsId";
    pub const COLUMNS_ARRAY: [&'static str; 6] = ["EmployeeContactsId","EmployeeId","ContactTypeId","ContactValue","IsPrimary","StatusId"];

    pub fn get_id(&self) -> Uuid {
        self.employee_contacts_id.clone()
    }

    pub fn get_args(&self) -> PgArguments {
        let mut args = PgArguments::default();
        let _ = args.add(self.employee_contacts_id.clone());
let _ = args.add(self.employee_id.clone());
let _ = args.add(self.contact_type_id.clone());
let _ = args.add(self.contact_value.clone());
let _ = args.add(self.is_primary.clone());
let _ = args.add(self.status_id.clone());

        args
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
