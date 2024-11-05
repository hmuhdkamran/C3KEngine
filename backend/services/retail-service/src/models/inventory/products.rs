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
    pub const PK: &'static str = "ProductId";
    pub const COLUMNS_ARRAY: [&'static str; 7] = ["ProductId","Name","SKU","ProductTypeId","BrandId","UnitId","StatusId"];

    pub fn get_id(&self) -> Uuid {
        self.product_id.clone()
    }

    pub fn get_args(&self) -> PgArguments {
        let mut args = PgArguments::default();
        let _ = args.add(self.product_id.clone());
let _ = args.add(self.name.clone());
let _ = args.add(self.sku.clone());
let _ = args.add(self.product_type_id.clone());
let _ = args.add(self.brand_id.clone());
let _ = args.add(self.unit_id.clone());
let _ = args.add(self.status_id.clone());

        args
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
