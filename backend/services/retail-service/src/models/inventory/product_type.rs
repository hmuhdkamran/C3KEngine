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
pub struct ProductType {
    pub product_typ_id: Uuid,
pub abbreviation: String,
pub full_name: String,
pub status_id: Uuid

}

impl ProductType {
    pub const TABLE: &'static str = r#""Inventory"."ProductType""#;
    pub const PK: &'static str = "ProductTypId";
    pub const COLUMNS_ARRAY: [&'static str; 4] = ["ProductTypId","Abbreviation","FullName","StatusId"];

    pub fn get_id(&self) -> Uuid {
        self.product_typ_id.clone()
    }

    pub fn get_args(&self) -> PgArguments {
        let mut args = PgArguments::default();
        let _ = args.add(self.product_typ_id.clone());
let _ = args.add(self.abbreviation.clone());
let _ = args.add(self.full_name.clone());
let _ = args.add(self.status_id.clone());

        args
    }

    pub fn new(product_typ_id: Uuid,abbreviation: String,full_name: String,status_id: Uuid) -> Self {
        Self {
            product_typ_id,
abbreviation,
full_name,
status_id

        }
    }
}

impl PartialEq for ProductType {
    fn eq(&self, other: &Self) -> bool {
        self.product_typ_id == other.product_typ_id
    }
}

impl Model for ProductType {
    fn from_row(row: &PgRow) -> ProductType
    where
        Self: Sized,
    {
        let product_typ_id = row.get("ProductTypId");
let abbreviation = row.get("Abbreviation");
let full_name = row.get("FullName");
let status_id = row.get("StatusId");


        Self {
            product_typ_id,
abbreviation,
full_name,
status_id

        }
    }
}
