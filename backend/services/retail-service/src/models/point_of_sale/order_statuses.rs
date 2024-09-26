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
pub struct OrderStatuses {
    pub order_status_id: Uuid,
pub abberviation: String,
pub full_name: String,
pub status_id: Uuid

}

impl OrderStatuses {
    pub const TABLE: &'static str = r#""PointOfSale"."OrderStatuses""#;
    pub const PK: &'static str = r#"OrderStatusId::TEXT=$1"#;
    pub const COLUMNS: &'static str = r#""OrderStatusId","Abberviation","FullName","StatusId""#;
    pub const COLUMNS_UPDATE: &'static str = r#""OrderStatusId"=$1,"Abberviation"=$2,"FullName"=$3,"StatusId"=$4 WHERE "OrderStatusId"=$1"#;

    pub fn get_id(&self) -> Uuid {
        self.order_status_id.clone()
    }

    pub fn new(order_status_id: Uuid,abberviation: String,full_name: String,status_id: Uuid) -> Self {
        Self {
            order_status_id,
abberviation,
full_name,
status_id

        }
    }
}

impl PartialEq for OrderStatuses {
    fn eq(&self, other: &Self) -> bool {
        self.order_status_id == other.order_status_id
    }
}

impl Model for OrderStatuses {
    fn from_row(row: &PgRow) -> OrderStatuses
    where
        Self: Sized,
    {
        let order_status_id = row.get("OrderStatusId");
let abberviation = row.get("Abberviation");
let full_name = row.get("FullName");
let status_id = row.get("StatusId");


        Self {
            order_status_id,
abberviation,
full_name,
status_id

        }
    }
}
