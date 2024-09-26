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
pub struct ProductPrices {
    pub product_price_id: Uuid,
pub standard_cost: f64,
pub purchase_price: f64,
pub whole_sale_price: f64,
pub retail_price: f64,
pub status_id: Uuid,
pub product_id: Uuid

}

impl ProductPrices {
    pub const TABLE: &'static str = r#""Inventory"."ProductPrices""#;
    pub const PK: &'static str = r#"ProductPriceId::TEXT=$1"#;
    pub const COLUMNS: &'static str = r#""ProductPriceId","StandardCost","PurchasePrice","WholeSalePrice","RetailPrice","StatusId","ProductId""#;
    pub const COLUMNS_UPDATE: &'static str = r#""ProductPriceId"=$1,"StandardCost"=$2,"PurchasePrice"=$3,"WholeSalePrice"=$4,"RetailPrice"=$5,"StatusId"=$6,"ProductId"=$7 WHERE "ProductPriceId"=$1"#;

    pub fn get_id(&self) -> Uuid {
        self.product_price_id.clone()
    }

    pub fn new(product_price_id: Uuid,standard_cost: f64,purchase_price: f64,whole_sale_price: f64,retail_price: f64,status_id: Uuid,product_id: Uuid) -> Self {
        Self {
            product_price_id,
standard_cost,
purchase_price,
whole_sale_price,
retail_price,
status_id,
product_id

        }
    }
}

impl PartialEq for ProductPrices {
    fn eq(&self, other: &Self) -> bool {
        self.product_price_id == other.product_price_id
    }
}

impl Model for ProductPrices {
    fn from_row(row: &PgRow) -> ProductPrices
    where
        Self: Sized,
    {
        let product_price_id = row.get("ProductPriceId");
let standard_cost = row.get("StandardCost");
let purchase_price = row.get("PurchasePrice");
let whole_sale_price = row.get("WholeSalePrice");
let retail_price = row.get("RetailPrice");
let status_id = row.get("StatusId");
let product_id = row.get("ProductId");


        Self {
            product_price_id,
standard_cost,
purchase_price,
whole_sale_price,
retail_price,
status_id,
product_id

        }
    }
}
