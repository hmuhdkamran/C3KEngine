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
pub struct PaymentMethods {
    pub payment_method_id: Uuid,
pub abberviation: String,
pub full_name: String,
pub status_id: Uuid

}

impl PaymentMethods {
    pub const TABLE: &'static str = r#""PointOfSale"."PaymentMethods""#;
    pub const PK: &'static str = r#"PaymentMethodId::TEXT=$1"#;
    pub const COLUMNS: &'static str = r#""PaymentMethodId","Abberviation","FullName","StatusId""#;
    pub const COLUMNS_UPDATE: &'static str = r#""PaymentMethodId"=$1,"Abberviation"=$2,"FullName"=$3,"StatusId"=$4 WHERE "PaymentMethodId"=$1"#;

    pub fn get_id(&self) -> Uuid {
        self.payment_method_id.clone()
    }

    pub fn new(payment_method_id: Uuid,abberviation: String,full_name: String,status_id: Uuid) -> Self {
        Self {
            payment_method_id,
abberviation,
full_name,
status_id

        }
    }
}

impl PartialEq for PaymentMethods {
    fn eq(&self, other: &Self) -> bool {
        self.payment_method_id == other.payment_method_id
    }
}

impl Model for PaymentMethods {
    fn from_row(row: &PgRow) -> PaymentMethods
    where
        Self: Sized,
    {
        let payment_method_id = row.get("PaymentMethodId");
let abberviation = row.get("Abberviation");
let full_name = row.get("FullName");
let status_id = row.get("StatusId");


        Self {
            payment_method_id,
abberviation,
full_name,
status_id

        }
    }
}
