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
    pub const PK: &'static str = r#"CustomerId::TEXT=$1"#;
    pub const COLUMNS: &'static str = r#""CustomerId","FirstName","MiddleName","LastName","StatusId","CustomerTypeId""#;
    pub const COLUMNS_UPDATE: &'static str = r#""CustomerId"=$1,"FirstName"=$2,"MiddleName"=$3,"LastName"=$4,"StatusId"=$5,"CustomerTypeId"=$6 WHERE "CustomerId"=$1"#;

    pub fn get_id(&self) -> Uuid {
        self.customer_id.clone()
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
