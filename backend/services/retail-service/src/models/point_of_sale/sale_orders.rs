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
pub struct SaleOrders {
    pub sale_order_id: Uuid,
pub order_no: String,
pub order_date: DateTime<Utc>,
pub reff_no: String,
pub order_status_id: Uuid,
pub order_type_id: Uuid,
pub customer_id: Uuid,
pub customer_type_id: Uuid,
pub branch_id: Uuid,
pub status_id: Uuid,
pub fbr_invoice_no: String

}

impl SaleOrders {
    pub const TABLE: &'static str = r#""PointOfSale"."SaleOrders""#;
    pub const PK: &'static str = r#"SaleOrderId::TEXT=$1"#;
    pub const COLUMNS: &'static str = r#""SaleOrderId","OrderNo","OrderDate","ReffNo","OrderStatusId","OrderTypeId","CustomerId","CustomerTypeId","BranchId","StatusId","FBRInvoiceNo""#;
    pub const COLUMNS_UPDATE: &'static str = r#""SaleOrderId"=$1,"OrderNo"=$2,"OrderDate"=$3,"ReffNo"=$4,"OrderStatusId"=$5,"OrderTypeId"=$6,"CustomerId"=$7,"CustomerTypeId"=$8,"BranchId"=$9,"StatusId"=$10,"FBRInvoiceNo"=$11 WHERE "SaleOrderId"=$1"#;

    pub fn get_id(&self) -> Uuid {
        self.sale_order_id.clone()
    }

    pub fn new(sale_order_id: Uuid,order_no: String,order_date: DateTime<Utc>,reff_no: String,order_status_id: Uuid,order_type_id: Uuid,customer_id: Uuid,customer_type_id: Uuid,branch_id: Uuid,status_id: Uuid,fbr_invoice_no: String) -> Self {
        Self {
            sale_order_id,
order_no,
order_date,
reff_no,
order_status_id,
order_type_id,
customer_id,
customer_type_id,
branch_id,
status_id,
fbr_invoice_no

        }
    }
}

impl PartialEq for SaleOrders {
    fn eq(&self, other: &Self) -> bool {
        self.sale_order_id == other.sale_order_id
    }
}

impl Model for SaleOrders {
    fn from_row(row: &PgRow) -> SaleOrders
    where
        Self: Sized,
    {
        let sale_order_id = row.get("SaleOrderId");
let order_no = row.get("OrderNo");
let order_date = row.get("OrderDate");
let reff_no = row.get("ReffNo");
let order_status_id = row.get("OrderStatusId");
let order_type_id = row.get("OrderTypeId");
let customer_id = row.get("CustomerId");
let customer_type_id = row.get("CustomerTypeId");
let branch_id = row.get("BranchId");
let status_id = row.get("StatusId");
let fbr_invoice_no = row.get("FBRInvoiceNo");


        Self {
            sale_order_id,
order_no,
order_date,
reff_no,
order_status_id,
order_type_id,
customer_id,
customer_type_id,
branch_id,
status_id,
fbr_invoice_no

        }
    }
}
