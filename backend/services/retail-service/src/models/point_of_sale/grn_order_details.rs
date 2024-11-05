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
pub struct GrnOrderDetails {
    pub grn_order_detail_id: Uuid,
pub grn_order_id: Uuid,
pub product_id: Uuid,
pub qunatity: f64,
pub warehouse_id: Uuid,
pub sale_price: f64,
pub purchase_price: f64,
pub whole_sale_price: f64,
pub net_price: f64,
pub status_id: Uuid

}

impl GrnOrderDetails {
    pub const TABLE: &'static str = r#""PointOfSale"."GrnOrderDetails""#;
    pub const PK: &'static str = "GrnOrderDetailId";
    pub const COLUMNS_ARRAY: [&'static str; 10] = ["GrnOrderDetailId","GrnOrderId","ProductId","Qunatity","WarehouseId","SalePrice","PurchasePrice","WholeSalePrice","NetPrice","StatusId"];

    pub fn get_id(&self) -> Uuid {
        self.grn_order_detail_id.clone()
    }

    pub fn get_args(&self) -> PgArguments {
        let mut args = PgArguments::default();
        let _ = args.add(self.grn_order_detail_id.clone());
let _ = args.add(self.grn_order_id.clone());
let _ = args.add(self.product_id.clone());
let _ = args.add(self.qunatity.clone());
let _ = args.add(self.warehouse_id.clone());
let _ = args.add(self.sale_price.clone());
let _ = args.add(self.purchase_price.clone());
let _ = args.add(self.whole_sale_price.clone());
let _ = args.add(self.net_price.clone());
let _ = args.add(self.status_id.clone());

        args
    }

    pub fn new(grn_order_detail_id: Uuid,grn_order_id: Uuid,product_id: Uuid,qunatity: f64,warehouse_id: Uuid,sale_price: f64,purchase_price: f64,whole_sale_price: f64,net_price: f64,status_id: Uuid) -> Self {
        Self {
            grn_order_detail_id,
grn_order_id,
product_id,
qunatity,
warehouse_id,
sale_price,
purchase_price,
whole_sale_price,
net_price,
status_id

        }
    }
}

impl PartialEq for GrnOrderDetails {
    fn eq(&self, other: &Self) -> bool {
        self.grn_order_detail_id == other.grn_order_detail_id
    }
}

impl Model for GrnOrderDetails {
    fn from_row(row: &PgRow) -> GrnOrderDetails
    where
        Self: Sized,
    {
        let grn_order_detail_id = row.get("GrnOrderDetailId");
let grn_order_id = row.get("GrnOrderId");
let product_id = row.get("ProductId");
let qunatity = row.get("Qunatity");
let warehouse_id = row.get("WarehouseId");
let sale_price = row.get("SalePrice");
let purchase_price = row.get("PurchasePrice");
let whole_sale_price = row.get("WholeSalePrice");
let net_price = row.get("NetPrice");
let status_id = row.get("StatusId");


        Self {
            grn_order_detail_id,
grn_order_id,
product_id,
qunatity,
warehouse_id,
sale_price,
purchase_price,
whole_sale_price,
net_price,
status_id

        }
    }
}
