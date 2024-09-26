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
pub struct Brands {
    pub brand_id: Uuid,
pub abberviation: String,
pub full_name: String,
pub status_id: Uuid

}

impl Brands {
    pub const TABLE: &'static str = r#""Inventory"."Brands""#;
    pub const PK: &'static str = r#"BrandId::TEXT=$1"#;
    pub const COLUMNS: &'static str = r#""BrandId","Abberviation","FullName","StatusId""#;
    pub const COLUMNS_UPDATE: &'static str = r#""BrandId"=$1,"Abberviation"=$2,"FullName"=$3,"StatusId"=$4 WHERE "BrandId"=$1"#;

    pub fn get_id(&self) -> Uuid {
        self.brand_id.clone()
    }

    pub fn new(brand_id: Uuid,abberviation: String,full_name: String,status_id: Uuid) -> Self {
        Self {
            brand_id,
abberviation,
full_name,
status_id

        }
    }
}

impl PartialEq for Brands {
    fn eq(&self, other: &Self) -> bool {
        self.brand_id == other.brand_id
    }
}

impl Model for Brands {
    fn from_row(row: &PgRow) -> Brands
    where
        Self: Sized,
    {
        let brand_id = row.get("BrandId");
let abberviation = row.get("Abberviation");
let full_name = row.get("FullName");
let status_id = row.get("StatusId");


        Self {
            brand_id,
abberviation,
full_name,
status_id

        }
    }
}
