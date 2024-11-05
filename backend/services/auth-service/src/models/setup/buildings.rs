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
pub struct Buildings {
    pub building_id: Uuid,
pub business_id: Uuid,
pub abbreviation: String,
pub full_name: String,
pub address: String,
pub area_id: Uuid,
pub building_area: i32,
pub status_id: Uuid,
pub ownership_status_id: Uuid

}

impl Buildings {
    pub const TABLE: &'static str = r#""Setup"."Buildings""#;
    pub const PK: &'static str = "BuildingId";
    pub const COLUMNS_ARRAY: [&'static str; 9] = ["BuildingId","BusinessId","Abbreviation","FullName","Address","AreaId","BuildingArea","StatusId","OwnershipStatusId"];

    pub fn get_id(&self) -> Uuid {
        self.building_id.clone()
    }

    pub fn get_args(&self) -> PgArguments {
        let mut args = PgArguments::default();
        let _ = args.add(self.building_id.clone());
let _ = args.add(self.business_id.clone());
let _ = args.add(self.abbreviation.clone());
let _ = args.add(self.full_name.clone());
let _ = args.add(self.address.clone());
let _ = args.add(self.area_id.clone());
let _ = args.add(self.building_area.clone());
let _ = args.add(self.status_id.clone());
let _ = args.add(self.ownership_status_id.clone());

        args
    }

    pub fn new(building_id: Uuid,business_id: Uuid,abbreviation: String,full_name: String,address: String,area_id: Uuid,building_area: i32,status_id: Uuid,ownership_status_id: Uuid) -> Self {
        Self {
            building_id,
business_id,
abbreviation,
full_name,
address,
area_id,
building_area,
status_id,
ownership_status_id

        }
    }
}

impl PartialEq for Buildings {
    fn eq(&self, other: &Self) -> bool {
        self.building_id == other.building_id
    }
}

impl Model for Buildings {
    fn from_row(row: &PgRow) -> Buildings
    where
        Self: Sized,
    {
        let building_id = row.get("BuildingId");
let business_id = row.get("BusinessId");
let abbreviation = row.get("Abbreviation");
let full_name = row.get("FullName");
let address = row.get("Address");
let area_id = row.get("AreaId");
let building_area = row.get("BuildingArea");
let status_id = row.get("StatusId");
let ownership_status_id = row.get("OwnershipStatusId");


        Self {
            building_id,
business_id,
abbreviation,
full_name,
address,
area_id,
building_area,
status_id,
ownership_status_id

        }
    }
}
