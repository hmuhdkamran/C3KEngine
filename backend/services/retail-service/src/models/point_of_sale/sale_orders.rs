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
    pub const PK: &'static str = "SaleOrderId";
    pub const COLUMNS_ARRAY: [&'static str; 11] = ["SaleOrderId","OrderNo","OrderDate","ReffNo","OrderStatusId","OrderTypeId","CustomerId","CustomerTypeId","BranchId","StatusId","FBRInvoiceNo"];

    pub fn get_id(&self) -> Uuid {
        self.sale_order_id.clone()
    }

    pub fn get_args(&self) -> PgArguments {
        let mut args = PgArguments::default();
        let _ = args.add(self.sale_order_id.clone());
let _ = args.add(self.order_no.clone());
let _ = args.add(self.order_date.clone());
let _ = args.add(self.reff_no.clone());
let _ = args.add(self.order_status_id.clone());
let _ = args.add(self.order_type_id.clone());
let _ = args.add(self.customer_id.clone());
let _ = args.add(self.customer_type_id.clone());
let _ = args.add(self.branch_id.clone());
let _ = args.add(self.status_id.clone());
let _ = args.add(self.fbr_invoice_no.clone());

        args
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
