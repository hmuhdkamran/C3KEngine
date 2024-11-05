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
pub struct ProductWarehouses {
    pub product_warehouse_id: Uuid,
pub warehouse_id: Uuid,
pub product_id: Uuid,
pub quantity: f64,
pub status_id: Uuid

}

impl ProductWarehouses {
    pub const TABLE: &'static str = r#""Inventory"."ProductWarehouses""#;
    pub const PK: &'static str = "ProductWarehouseId";
    pub const COLUMNS_ARRAY: [&'static str; 5] = ["ProductWarehouseId","WarehouseId","ProductId","Quantity","StatusId"];

    pub fn get_id(&self) -> Uuid {
        self.product_warehouse_id.clone()
    }

    pub fn get_args(&self) -> PgArguments {
        let mut args = PgArguments::default();
        let _ = args.add(self.product_warehouse_id.clone());
let _ = args.add(self.warehouse_id.clone());
let _ = args.add(self.product_id.clone());
let _ = args.add(self.quantity.clone());
let _ = args.add(self.status_id.clone());

        args
    }

    pub fn new(product_warehouse_id: Uuid,warehouse_id: Uuid,product_id: Uuid,quantity: f64,status_id: Uuid) -> Self {
        Self {
            product_warehouse_id,
warehouse_id,
product_id,
quantity,
status_id

        }
    }
}

impl PartialEq for ProductWarehouses {
    fn eq(&self, other: &Self) -> bool {
        self.product_warehouse_id == other.product_warehouse_id
    }
}

impl Model for ProductWarehouses {
    fn from_row(row: &PgRow) -> ProductWarehouses
    where
        Self: Sized,
    {
        let product_warehouse_id = row.get("ProductWarehouseId");
let warehouse_id = row.get("WarehouseId");
let product_id = row.get("ProductId");
let quantity = row.get("Quantity");
let status_id = row.get("StatusId");


        Self {
            product_warehouse_id,
warehouse_id,
product_id,
quantity,
status_id

        }
    }
}
