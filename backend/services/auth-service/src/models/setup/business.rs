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
pub struct Business {
    pub business_id: Uuid,
pub abbreviation: String,
pub full_name: String,
pub address: String,
pub area_id: Uuid,
pub status_id: Uuid

}

impl Business {
    pub const TABLE: &'static str = r#""Setup"."Business""#;
    pub const PK: &'static str = "BusinessId";
    pub const COLUMNS_ARRAY: [&'static str; 6] = ["BusinessId","Abbreviation","FullName","Address","AreaId","StatusId"];

    pub fn get_id(&self) -> Uuid {
        self.business_id.clone()
    }

    pub fn get_args(&self) -> PgArguments {
        let mut args = PgArguments::default();
        let _ = args.add(self.business_id.clone());
let _ = args.add(self.abbreviation.clone());
let _ = args.add(self.full_name.clone());
let _ = args.add(self.address.clone());
let _ = args.add(self.area_id.clone());
let _ = args.add(self.status_id.clone());

        args
    }

    pub fn new(business_id: Uuid,abbreviation: String,full_name: String,address: String,area_id: Uuid,status_id: Uuid) -> Self {
        Self {
            business_id,
abbreviation,
full_name,
address,
area_id,
status_id

        }
    }
}

impl PartialEq for Business {
    fn eq(&self, other: &Self) -> bool {
        self.business_id == other.business_id
    }
}

impl Model for Business {
    fn from_row(row: &PgRow) -> Business
    where
        Self: Sized,
    {
        let business_id = row.get("BusinessId");
let abbreviation = row.get("Abbreviation");
let full_name = row.get("FullName");
let address = row.get("Address");
let area_id = row.get("AreaId");
let status_id = row.get("StatusId");


        Self {
            business_id,
abbreviation,
full_name,
address,
area_id,
status_id

        }
    }
}
