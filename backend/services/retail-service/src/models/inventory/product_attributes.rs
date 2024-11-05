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
pub struct ProductAttributes {
    pub product_attribute_id: Uuid,
pub product_id: Uuid,
pub attirbute_id: Uuid,
pub attribute_value: String,
pub status_id: Uuid

}

impl ProductAttributes {
    pub const TABLE: &'static str = r#""Inventory"."ProductAttributes""#;
    pub const PK: &'static str = "ProductAttributeId";
    pub const COLUMNS_ARRAY: [&'static str; 5] = ["ProductAttributeId","ProductId","AttirbuteId","AttributeValue","StatusId"];

    pub fn get_id(&self) -> Uuid {
        self.product_attribute_id.clone()
    }

    pub fn get_args(&self) -> PgArguments {
        let mut args = PgArguments::default();
        let _ = args.add(self.product_attribute_id.clone());
let _ = args.add(self.product_id.clone());
let _ = args.add(self.attirbute_id.clone());
let _ = args.add(self.attribute_value.clone());
let _ = args.add(self.status_id.clone());

        args
    }

    pub fn new(product_attribute_id: Uuid,product_id: Uuid,attirbute_id: Uuid,attribute_value: String,status_id: Uuid) -> Self {
        Self {
            product_attribute_id,
product_id,
attirbute_id,
attribute_value,
status_id

        }
    }
}

impl PartialEq for ProductAttributes {
    fn eq(&self, other: &Self) -> bool {
        self.product_attribute_id == other.product_attribute_id
    }
}

impl Model for ProductAttributes {
    fn from_row(row: &PgRow) -> ProductAttributes
    where
        Self: Sized,
    {
        let product_attribute_id = row.get("ProductAttributeId");
let product_id = row.get("ProductId");
let attirbute_id = row.get("AttirbuteId");
let attribute_value = row.get("AttributeValue");
let status_id = row.get("StatusId");


        Self {
            product_attribute_id,
product_id,
attirbute_id,
attribute_value,
status_id

        }
    }
}
