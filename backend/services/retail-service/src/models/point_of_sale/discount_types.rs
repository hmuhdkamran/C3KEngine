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
pub struct DiscountTypes {
    pub discount_type_id: Uuid,
pub abberviation: String,
pub full_name: String,
pub status_id: Uuid

}

impl DiscountTypes {
    pub const TABLE: &'static str = r#""PointOfSale"."DiscountTypes""#;
    pub const PK: &'static str = r#"DiscountTypeId::TEXT=$1"#;
    pub const COLUMNS: &'static str = r#""DiscountTypeId","Abberviation","FullName","StatusId""#;
    pub const COLUMNS_UPDATE: &'static str = r#""DiscountTypeId"=$1,"Abberviation"=$2,"FullName"=$3,"StatusId"=$4 WHERE "DiscountTypeId"=$1"#;

    pub fn get_id(&self) -> Uuid {
        self.discount_type_id.clone()
    }

    pub fn new(discount_type_id: Uuid,abberviation: String,full_name: String,status_id: Uuid) -> Self {
        Self {
            discount_type_id,
abberviation,
full_name,
status_id

        }
    }
}

impl PartialEq for DiscountTypes {
    fn eq(&self, other: &Self) -> bool {
        self.discount_type_id == other.discount_type_id
    }
}

impl Model for DiscountTypes {
    fn from_row(row: &PgRow) -> DiscountTypes
    where
        Self: Sized,
    {
        let discount_type_id = row.get("DiscountTypeId");
let abberviation = row.get("Abberviation");
let full_name = row.get("FullName");
let status_id = row.get("StatusId");


        Self {
            discount_type_id,
abberviation,
full_name,
status_id

        }
    }
}
