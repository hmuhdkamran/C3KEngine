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
pub struct SaleOrderPayments {
    pub sale_order_payment_id: Uuid,
pub payment_type_id: Uuid,
pub sale_order_id: Uuid,
pub net_amount: f64,
pub pay_amount: f64,
pub balance: f64,
pub status_id: Uuid,
pub payment_term_id: Uuid

}

impl SaleOrderPayments {
    pub const TABLE: &'static str = r#""PointOfSale"."SaleOrderPayments""#;
    pub const PK: &'static str = "SaleOrderPaymentId";
    pub const COLUMNS_ARRAY: [&'static str; 8] = ["SaleOrderPaymentId","PaymentTypeId","SaleOrderId","NetAmount","PayAmount","Balance","StatusId","PaymentTermId"];

    pub fn get_id(&self) -> Uuid {
        self.sale_order_payment_id.clone()
    }

    pub fn get_args(&self) -> PgArguments {
        let mut args = PgArguments::default();
        let _ = args.add(self.sale_order_payment_id.clone());
let _ = args.add(self.payment_type_id.clone());
let _ = args.add(self.sale_order_id.clone());
let _ = args.add(self.net_amount.clone());
let _ = args.add(self.pay_amount.clone());
let _ = args.add(self.balance.clone());
let _ = args.add(self.status_id.clone());
let _ = args.add(self.payment_term_id.clone());

        args
    }

    pub fn new(sale_order_payment_id: Uuid,payment_type_id: Uuid,sale_order_id: Uuid,net_amount: f64,pay_amount: f64,balance: f64,status_id: Uuid,payment_term_id: Uuid) -> Self {
        Self {
            sale_order_payment_id,
payment_type_id,
sale_order_id,
net_amount,
pay_amount,
balance,
status_id,
payment_term_id

        }
    }
}

impl PartialEq for SaleOrderPayments {
    fn eq(&self, other: &Self) -> bool {
        self.sale_order_payment_id == other.sale_order_payment_id
    }
}

impl Model for SaleOrderPayments {
    fn from_row(row: &PgRow) -> SaleOrderPayments
    where
        Self: Sized,
    {
        let sale_order_payment_id = row.get("SaleOrderPaymentId");
let payment_type_id = row.get("PaymentTypeId");
let sale_order_id = row.get("SaleOrderId");
let net_amount = row.get("NetAmount");
let pay_amount = row.get("PayAmount");
let balance = row.get("Balance");
let status_id = row.get("StatusId");
let payment_term_id = row.get("PaymentTermId");


        Self {
            sale_order_payment_id,
payment_type_id,
sale_order_id,
net_amount,
pay_amount,
balance,
status_id,
payment_term_id

        }
    }
}
