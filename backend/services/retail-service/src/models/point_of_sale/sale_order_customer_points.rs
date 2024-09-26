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
pub struct SaleOrderCustomerPoints {
    pub sale_order_customer_point_id: Uuid,
pub customer_id: Uuid,
pub sale_order_id: Uuid,
pub net_amount: f64,
pub point_awards: f64,
pub redeem_points: f64,
pub balance_point: f64,
pub status_id: Uuid

}

impl SaleOrderCustomerPoints {
    pub const TABLE: &'static str = r#""PointOfSale"."SaleOrderCustomerPoints""#;
    pub const PK: &'static str = r#"SaleOrderCustomerPointId::TEXT=$1"#;
    pub const COLUMNS: &'static str = r#""SaleOrderCustomerPointId","CustomerId","SaleOrderId","NetAmount","PointAwards","RedeemPoints","BalancePoint","StatusId""#;
    pub const COLUMNS_UPDATE: &'static str = r#""SaleOrderCustomerPointId"=$1,"CustomerId"=$2,"SaleOrderId"=$3,"NetAmount"=$4,"PointAwards"=$5,"RedeemPoints"=$6,"BalancePoint"=$7,"StatusId"=$8 WHERE "SaleOrderCustomerPointId"=$1"#;

    pub fn get_id(&self) -> Uuid {
        self.sale_order_customer_point_id.clone()
    }

    pub fn new(sale_order_customer_point_id: Uuid,customer_id: Uuid,sale_order_id: Uuid,net_amount: f64,point_awards: f64,redeem_points: f64,balance_point: f64,status_id: Uuid) -> Self {
        Self {
            sale_order_customer_point_id,
customer_id,
sale_order_id,
net_amount,
point_awards,
redeem_points,
balance_point,
status_id

        }
    }
}

impl PartialEq for SaleOrderCustomerPoints {
    fn eq(&self, other: &Self) -> bool {
        self.sale_order_customer_point_id == other.sale_order_customer_point_id
    }
}

impl Model for SaleOrderCustomerPoints {
    fn from_row(row: &PgRow) -> SaleOrderCustomerPoints
    where
        Self: Sized,
    {
        let sale_order_customer_point_id = row.get("SaleOrderCustomerPointId");
let customer_id = row.get("CustomerId");
let sale_order_id = row.get("SaleOrderId");
let net_amount = row.get("NetAmount");
let point_awards = row.get("PointAwards");
let redeem_points = row.get("RedeemPoints");
let balance_point = row.get("BalancePoint");
let status_id = row.get("StatusId");


        Self {
            sale_order_customer_point_id,
customer_id,
sale_order_id,
net_amount,
point_awards,
redeem_points,
balance_point,
status_id

        }
    }
}
