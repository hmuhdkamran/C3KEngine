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
pub struct Products {
    pub product_id: Uuid,
pub name: String,
pub sku: String,
pub product_type_id: Uuid,
pub brand_id: Uuid,
pub unit_id: Uuid,
pub status_id: Uuid

}

impl Products {
    pub const TABLE: &'static str = r#""Inventory"."Products""#;
    pub const PK: &'static str = r#"ProductId::TEXT=$1"#;
    pub const COLUMNS: &'static str = r#""ProductId","Name","SKU","ProductTypeId","BrandId","UnitId","StatusId""#;
    pub const COLUMNS_UPDATE: &'static str = r#""ProductId"=$1,"Name"=$2,"SKU"=$3,"ProductTypeId"=$4,"BrandId"=$5,"UnitId"=$6,"StatusId"=$7 WHERE "ProductId"=$1"#;

    pub fn get_id(&self) -> Uuid {
        self.product_id.clone()
    }

    pub fn new(product_id: Uuid,name: String,sku: String,product_type_id: Uuid,brand_id: Uuid,unit_id: Uuid,status_id: Uuid) -> Self {
        Self {
            product_id,
name,
sku,
product_type_id,
brand_id,
unit_id,
status_id

        }
    }
}

impl PartialEq for Products {
    fn eq(&self, other: &Self) -> bool {
        self.product_id == other.product_id
    }
}

impl Model for Products {
    fn from_row(row: &PgRow) -> Products
    where
        Self: Sized,
    {
        let product_id = row.get("ProductId");
let name = row.get("Name");
let sku = row.get("SKU");
let product_type_id = row.get("ProductTypeId");
let brand_id = row.get("BrandId");
let unit_id = row.get("UnitId");
let status_id = row.get("StatusId");


        Self {
            product_id,
name,
sku,
product_type_id,
brand_id,
unit_id,
status_id

        }
    }
}
