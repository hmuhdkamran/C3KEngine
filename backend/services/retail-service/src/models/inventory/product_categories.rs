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
pub struct ProductCategories {
    pub product_category_id: Uuid,
pub category_id: Uuid,
pub status_id: Uuid,
pub product_id: Uuid

}

impl ProductCategories {
    pub const TABLE: &'static str = r#""Inventory"."ProductCategories""#;
    pub const PK: &'static str = "ProductCategoryId";
    pub const COLUMNS_ARRAY: [&'static str; 4] = ["ProductCategoryId","CategoryId","StatusId","ProductId"];

    pub fn get_id(&self) -> Uuid {
        self.product_category_id.clone()
    }

    pub fn get_args(&self) -> PgArguments {
        let mut args = PgArguments::default();
        let _ = args.add(self.product_category_id.clone());
let _ = args.add(self.category_id.clone());
let _ = args.add(self.status_id.clone());
let _ = args.add(self.product_id.clone());

        args
    }

    pub fn new(product_category_id: Uuid,category_id: Uuid,status_id: Uuid,product_id: Uuid) -> Self {
        Self {
            product_category_id,
category_id,
status_id,
product_id

        }
    }
}

impl PartialEq for ProductCategories {
    fn eq(&self, other: &Self) -> bool {
        self.product_category_id == other.product_category_id
    }
}

impl Model for ProductCategories {
    fn from_row(row: &PgRow) -> ProductCategories
    where
        Self: Sized,
    {
        let product_category_id = row.get("ProductCategoryId");
let category_id = row.get("CategoryId");
let status_id = row.get("StatusId");
let product_id = row.get("ProductId");


        Self {
            product_category_id,
category_id,
status_id,
product_id

        }
    }
}
