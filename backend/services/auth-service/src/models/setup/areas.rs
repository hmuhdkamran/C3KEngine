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
pub struct Areas {
    pub area_id: Uuid,
pub city_id: Uuid,
pub abbreviation: String,
pub full_name: String,
pub status_id: Uuid

}

impl Areas {
    pub const TABLE: &'static str = r#""Setup"."Areas""#;
    pub const PK: &'static str = "AreaId";
    pub const COLUMNS_ARRAY: [&'static str; 5] = ["AreaId","CityId","Abbreviation","FullName","StatusId"];

    pub fn get_id(&self) -> Uuid {
        self.area_id.clone()
    }

    pub fn get_args(&self) -> PgArguments {
        let mut args = PgArguments::default();
        let _ = args.add(self.area_id.clone());
let _ = args.add(self.city_id.clone());
let _ = args.add(self.abbreviation.clone());
let _ = args.add(self.full_name.clone());
let _ = args.add(self.status_id.clone());

        args
    }

    pub fn new(area_id: Uuid,city_id: Uuid,abbreviation: String,full_name: String,status_id: Uuid) -> Self {
        Self {
            area_id,
city_id,
abbreviation,
full_name,
status_id

        }
    }
}

impl PartialEq for Areas {
    fn eq(&self, other: &Self) -> bool {
        self.area_id == other.area_id
    }
}

impl Model for Areas {
    fn from_row(row: &PgRow) -> Areas
    where
        Self: Sized,
    {
        let area_id = row.get("AreaId");
let city_id = row.get("CityId");
let abbreviation = row.get("Abbreviation");
let full_name = row.get("FullName");
let status_id = row.get("StatusId");


        Self {
            area_id,
city_id,
abbreviation,
full_name,
status_id

        }
    }
}
