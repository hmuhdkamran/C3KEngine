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
pub struct BuildingBrandMaps {
    pub building_brand_map_id: Uuid,
pub building_id: Uuid,
pub brand_id: Uuid,
pub code: String,
pub status_id: Uuid

}

impl BuildingBrandMaps {
    pub const TABLE: &'static str = r#""Setup"."BuildingBrandMaps""#;
    pub const PK: &'static str = "BuildingBrandMapId";
    pub const COLUMNS_ARRAY: [&'static str; 5] = ["BuildingBrandMapId","BuildingId","BrandId","Code","StatusId"];

    pub fn get_id(&self) -> Uuid {
        self.building_brand_map_id.clone()
    }

    pub fn get_args(&self) -> PgArguments {
        let mut args = PgArguments::default();
        let _ = args.add(self.building_brand_map_id.clone());
let _ = args.add(self.building_id.clone());
let _ = args.add(self.brand_id.clone());
let _ = args.add(self.code.clone());
let _ = args.add(self.status_id.clone());

        args
    }

    pub fn new(building_brand_map_id: Uuid,building_id: Uuid,brand_id: Uuid,code: String,status_id: Uuid) -> Self {
        Self {
            building_brand_map_id,
building_id,
brand_id,
code,
status_id

        }
    }
}

impl PartialEq for BuildingBrandMaps {
    fn eq(&self, other: &Self) -> bool {
        self.building_brand_map_id == other.building_brand_map_id
    }
}

impl Model for BuildingBrandMaps {
    fn from_row(row: &PgRow) -> BuildingBrandMaps
    where
        Self: Sized,
    {
        let building_brand_map_id = row.get("BuildingBrandMapId");
let building_id = row.get("BuildingId");
let brand_id = row.get("BrandId");
let code = row.get("Code");
let status_id = row.get("StatusId");


        Self {
            building_brand_map_id,
building_id,
brand_id,
code,
status_id

        }
    }
}
