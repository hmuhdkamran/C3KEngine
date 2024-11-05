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
pub struct Customers {
    pub customer_id: Uuid,
pub first_name: String,
pub middle_name: String,
pub last_name: String,
pub status_id: Uuid,
pub customer_type_id: Uuid

}

impl Customers {
    pub const TABLE: &'static str = r#""Customer"."Customers""#;
    pub const PK: &'static str = "CustomerId";
    pub const COLUMNS_ARRAY: [&'static str; 6] = ["CustomerId","FirstName","MiddleName","LastName","StatusId","CustomerTypeId"];

    pub fn get_id(&self) -> Uuid {
        self.customer_id.clone()
    }

    pub fn get_args(&self) -> PgArguments {
        let mut args = PgArguments::default();
        let _ = args.add(self.customer_id.clone());
let _ = args.add(self.first_name.clone());
let _ = args.add(self.middle_name.clone());
let _ = args.add(self.last_name.clone());
let _ = args.add(self.status_id.clone());
let _ = args.add(self.customer_type_id.clone());

        args
    }

    pub fn new(customer_id: Uuid,first_name: String,middle_name: String,last_name: String,status_id: Uuid,customer_type_id: Uuid) -> Self {
        Self {
            customer_id,
first_name,
middle_name,
last_name,
status_id,
customer_type_id

        }
    }
}

impl PartialEq for Customers {
    fn eq(&self, other: &Self) -> bool {
        self.customer_id == other.customer_id
    }
}

impl Model for Customers {
    fn from_row(row: &PgRow) -> Customers
    where
        Self: Sized,
    {
        let customer_id = row.get("CustomerId");
let first_name = row.get("FirstName");
let middle_name = row.get("MiddleName");
let last_name = row.get("LastName");
let status_id = row.get("StatusId");
let customer_type_id = row.get("CustomerTypeId");


        Self {
            customer_id,
first_name,
middle_name,
last_name,
status_id,
customer_type_id

        }
    }
}
