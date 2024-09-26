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
pub struct GrnOrders {
    pub grn_order_id: Uuid,
pub order_date: DateTime<Utc>,
pub bill_no: String,
pub reff_no: String,
pub order_status_id: Uuid,
pub branch_id: Uuid,
pub notes: String,
pub status_id: Uuid

}

impl GrnOrders {
    pub const TABLE: &'static str = r#""PointOfSale"."GrnOrders""#;
    pub const PK: &'static str = r#"GrnOrderId::TEXT=$1"#;
    pub const COLUMNS: &'static str = r#""GrnOrderId","OrderDate","BillNo","ReffNo","OrderStatusId","BranchId","Notes","StatusId""#;
    pub const COLUMNS_UPDATE: &'static str = r#""GrnOrderId"=$1,"OrderDate"=$2,"BillNo"=$3,"ReffNo"=$4,"OrderStatusId"=$5,"BranchId"=$6,"Notes"=$7,"StatusId"=$8 WHERE "GrnOrderId"=$1"#;

    pub fn get_id(&self) -> Uuid {
        self.grn_order_id.clone()
    }

    pub fn new(grn_order_id: Uuid,order_date: DateTime<Utc>,bill_no: String,reff_no: String,order_status_id: Uuid,branch_id: Uuid,notes: String,status_id: Uuid) -> Self {
        Self {
            grn_order_id,
order_date,
bill_no,
reff_no,
order_status_id,
branch_id,
notes,
status_id

        }
    }
}

impl PartialEq for GrnOrders {
    fn eq(&self, other: &Self) -> bool {
        self.grn_order_id == other.grn_order_id
    }
}

impl Model for GrnOrders {
    fn from_row(row: &PgRow) -> GrnOrders
    where
        Self: Sized,
    {
        let grn_order_id = row.get("GrnOrderId");
let order_date = row.get("OrderDate");
let bill_no = row.get("BillNo");
let reff_no = row.get("ReffNo");
let order_status_id = row.get("OrderStatusId");
let branch_id = row.get("BranchId");
let notes = row.get("Notes");
let status_id = row.get("StatusId");


        Self {
            grn_order_id,
order_date,
bill_no,
reff_no,
order_status_id,
branch_id,
notes,
status_id

        }
    }
}
