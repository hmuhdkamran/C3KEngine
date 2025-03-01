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
pub struct Warehouses {
    pub warehouse_id: Uuid,
pub abbreviation: String,
pub full_name: String,
pub location_id: Uuid,
pub status_id: Uuid

}

impl Warehouses {
    pub const TABLE: &'static str = r#""Setup"."Warehouses""#;
    pub const PK: &'static str = "WarehouseId";
    pub const COLUMNS_ARRAY: [&'static str; 5] = ["WarehouseId","Abbreviation","FullName","LocationId","StatusId"];

    pub fn get_id(&self) -> Uuid {
        self.warehouse_id.clone()
    }

    pub fn get_args(&self) -> PgArguments {
        let mut args = PgArguments::default();
        let _ = args.add(self.warehouse_id.clone());
let _ = args.add(self.abbreviation.clone());
let _ = args.add(self.full_name.clone());
let _ = args.add(self.location_id.clone());
let _ = args.add(self.status_id.clone());

        args
    }

    pub fn new(warehouse_id: Uuid,abbreviation: String,full_name: String,location_id: Uuid,status_id: Uuid) -> Self {
        Self {
            warehouse_id,
abbreviation,
full_name,
location_id,
status_id

        }
    }
}

impl PartialEq for Warehouses {
    fn eq(&self, other: &Self) -> bool {
        self.warehouse_id == other.warehouse_id
    }
}

impl Model for Warehouses {
    fn from_row(row: &PgRow) -> Warehouses
    where
        Self: Sized,
    {
        let warehouse_id = row.get("WarehouseId");
let abbreviation = row.get("Abbreviation");
let full_name = row.get("FullName");
let location_id = row.get("LocationId");
let status_id = row.get("StatusId");


        Self {
            warehouse_id,
abbreviation,
full_name,
location_id,
status_id

        }
    }
}
