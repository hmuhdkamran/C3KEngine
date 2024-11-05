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
pub struct OrderTypes {
    pub order_type_id: Uuid,
pub abberviation: String,
pub full_name: String,
pub status_id: Uuid

}

impl OrderTypes {
    pub const TABLE: &'static str = r#""PointOfSale"."OrderTypes""#;
    pub const PK: &'static str = "OrderTypeId";
    pub const COLUMNS_ARRAY: [&'static str; 4] = ["OrderTypeId","Abberviation","FullName","StatusId"];

    pub fn get_id(&self) -> Uuid {
        self.order_type_id.clone()
    }

    pub fn get_args(&self) -> PgArguments {
        let mut args = PgArguments::default();
        let _ = args.add(self.order_type_id.clone());
let _ = args.add(self.abberviation.clone());
let _ = args.add(self.full_name.clone());
let _ = args.add(self.status_id.clone());

        args
    }

    pub fn new(order_type_id: Uuid,abberviation: String,full_name: String,status_id: Uuid) -> Self {
        Self {
            order_type_id,
abberviation,
full_name,
status_id

        }
    }
}

impl PartialEq for OrderTypes {
    fn eq(&self, other: &Self) -> bool {
        self.order_type_id == other.order_type_id
    }
}

impl Model for OrderTypes {
    fn from_row(row: &PgRow) -> OrderTypes
    where
        Self: Sized,
    {
        let order_type_id = row.get("OrderTypeId");
let abberviation = row.get("Abberviation");
let full_name = row.get("FullName");
let status_id = row.get("StatusId");


        Self {
            order_type_id,
abberviation,
full_name,
status_id

        }
    }
}
