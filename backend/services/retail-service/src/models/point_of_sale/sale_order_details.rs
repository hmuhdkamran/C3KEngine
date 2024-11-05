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
pub struct SaleOrderDetails {
    pub sale_order_detail_id: Uuid,
pub sale_order_id: Uuid,
pub product_id: Uuid,
pub purchase_price: f64,
pub wholesale_price: f64,
pub sale_price: f64,
pub quantity: f64,
pub discount_type_id: Uuid,
pub discount_value: f64,
pub discount_amount: f64,
pub tax_rate: f64,
pub tax_value: f64,
pub sub_total: f64,
pub net_total: f64,
pub status_id: Uuid,
pub campaign_id: Uuid

}

impl SaleOrderDetails {
    pub const TABLE: &'static str = r#""PointOfSale"."SaleOrderDetails""#;
    pub const PK: &'static str = "SaleOrderDetailId";
    pub const COLUMNS_ARRAY: [&'static str; 16] = ["SaleOrderDetailId","SaleOrderId","ProductId","PurchasePrice","WholesalePrice","SalePrice","Quantity","DiscountTypeId","DiscountValue","DiscountAmount","TaxRate","TaxValue","SubTotal","NetTotal","StatusId","CampaignId"];

    pub fn get_id(&self) -> Uuid {
        self.sale_order_detail_id.clone()
    }

    pub fn get_args(&self) -> PgArguments {
        let mut args = PgArguments::default();
        let _ = args.add(self.sale_order_detail_id.clone());
let _ = args.add(self.sale_order_id.clone());
let _ = args.add(self.product_id.clone());
let _ = args.add(self.purchase_price.clone());
let _ = args.add(self.wholesale_price.clone());
let _ = args.add(self.sale_price.clone());
let _ = args.add(self.quantity.clone());
let _ = args.add(self.discount_type_id.clone());
let _ = args.add(self.discount_value.clone());
let _ = args.add(self.discount_amount.clone());
let _ = args.add(self.tax_rate.clone());
let _ = args.add(self.tax_value.clone());
let _ = args.add(self.sub_total.clone());
let _ = args.add(self.net_total.clone());
let _ = args.add(self.status_id.clone());
let _ = args.add(self.campaign_id.clone());

        args
    }

    pub fn new(sale_order_detail_id: Uuid,sale_order_id: Uuid,product_id: Uuid,purchase_price: f64,wholesale_price: f64,sale_price: f64,quantity: f64,discount_type_id: Uuid,discount_value: f64,discount_amount: f64,tax_rate: f64,tax_value: f64,sub_total: f64,net_total: f64,status_id: Uuid,campaign_id: Uuid) -> Self {
        Self {
            sale_order_detail_id,
sale_order_id,
product_id,
purchase_price,
wholesale_price,
sale_price,
quantity,
discount_type_id,
discount_value,
discount_amount,
tax_rate,
tax_value,
sub_total,
net_total,
status_id,
campaign_id

        }
    }
}

impl PartialEq for SaleOrderDetails {
    fn eq(&self, other: &Self) -> bool {
        self.sale_order_detail_id == other.sale_order_detail_id
    }
}

impl Model for SaleOrderDetails {
    fn from_row(row: &PgRow) -> SaleOrderDetails
    where
        Self: Sized,
    {
        let sale_order_detail_id = row.get("SaleOrderDetailId");
let sale_order_id = row.get("SaleOrderId");
let product_id = row.get("ProductId");
let purchase_price = row.get("PurchasePrice");
let wholesale_price = row.get("WholesalePrice");
let sale_price = row.get("SalePrice");
let quantity = row.get("Quantity");
let discount_type_id = row.get("DiscountTypeId");
let discount_value = row.get("DiscountValue");
let discount_amount = row.get("DiscountAmount");
let tax_rate = row.get("TaxRate");
let tax_value = row.get("TaxValue");
let sub_total = row.get("SubTotal");
let net_total = row.get("NetTotal");
let status_id = row.get("StatusId");
let campaign_id = row.get("CampaignId");


        Self {
            sale_order_detail_id,
sale_order_id,
product_id,
purchase_price,
wholesale_price,
sale_price,
quantity,
discount_type_id,
discount_value,
discount_amount,
tax_rate,
tax_value,
sub_total,
net_total,
status_id,
campaign_id

        }
    }
}
