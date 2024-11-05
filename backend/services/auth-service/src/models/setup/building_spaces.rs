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
pub struct BuildingSpaces {
    pub building_space_id: Uuid,
pub building_id: Uuid,
pub floor_id: Uuid,
pub space_type_id: Uuid,
pub full_name: String,
pub status_id: Uuid

}

impl BuildingSpaces {
    pub const TABLE: &'static str = r#""Setup"."BuildingSpaces""#;
    pub const PK: &'static str = "BuildingSpaceId";
    pub const COLUMNS_ARRAY: [&'static str; 6] = ["BuildingSpaceId","BuildingId","FloorId","SpaceTypeId","FullName","StatusId"];

    pub fn get_id(&self) -> Uuid {
        self.building_space_id.clone()
    }

    pub fn get_args(&self) -> PgArguments {
        let mut args = PgArguments::default();
        let _ = args.add(self.building_space_id.clone());
let _ = args.add(self.building_id.clone());
let _ = args.add(self.floor_id.clone());
let _ = args.add(self.space_type_id.clone());
let _ = args.add(self.full_name.clone());
let _ = args.add(self.status_id.clone());

        args
    }

    pub fn new(building_space_id: Uuid,building_id: Uuid,floor_id: Uuid,space_type_id: Uuid,full_name: String,status_id: Uuid) -> Self {
        Self {
            building_space_id,
building_id,
floor_id,
space_type_id,
full_name,
status_id

        }
    }
}

impl PartialEq for BuildingSpaces {
    fn eq(&self, other: &Self) -> bool {
        self.building_space_id == other.building_space_id
    }
}

impl Model for BuildingSpaces {
    fn from_row(row: &PgRow) -> BuildingSpaces
    where
        Self: Sized,
    {
        let building_space_id = row.get("BuildingSpaceId");
let building_id = row.get("BuildingId");
let floor_id = row.get("FloorId");
let space_type_id = row.get("SpaceTypeId");
let full_name = row.get("FullName");
let status_id = row.get("StatusId");


        Self {
            building_space_id,
building_id,
floor_id,
space_type_id,
full_name,
status_id

        }
    }
}
