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
pub struct Locations {
    pub location_id: Uuid,
pub abbreviation: String,
pub full_name: String,
pub address: String,
pub longitude: f64,
pub latitude: f64,
pub city_id: Uuid,
pub status_id: Uuid

}

impl Locations {
    pub const TABLE: &'static str = r#""Setup"."Locations""#;
    pub const PK: &'static str = "LocationId";
    pub const COLUMNS_ARRAY: [&'static str; 8] = ["LocationId","Abbreviation","FullName","Address","Longitude","Latitude","CityId","StatusId"];

    pub fn get_id(&self) -> Uuid {
        self.location_id.clone()
    }

    pub fn get_args(&self) -> PgArguments {
        let mut args = PgArguments::default();
        let _ = args.add(self.location_id.clone());
let _ = args.add(self.abbreviation.clone());
let _ = args.add(self.full_name.clone());
let _ = args.add(self.address.clone());
let _ = args.add(self.longitude.clone());
let _ = args.add(self.latitude.clone());
let _ = args.add(self.city_id.clone());
let _ = args.add(self.status_id.clone());

        args
    }

    pub fn new(location_id: Uuid,abbreviation: String,full_name: String,address: String,longitude: f64,latitude: f64,city_id: Uuid,status_id: Uuid) -> Self {
        Self {
            location_id,
abbreviation,
full_name,
address,
longitude,
latitude,
city_id,
status_id

        }
    }
}

impl PartialEq for Locations {
    fn eq(&self, other: &Self) -> bool {
        self.location_id == other.location_id
    }
}

impl Model for Locations {
    fn from_row(row: &PgRow) -> Locations
    where
        Self: Sized,
    {
        let location_id = row.get("LocationId");
let abbreviation = row.get("Abbreviation");
let full_name = row.get("FullName");
let address = row.get("Address");
let longitude = row.get("Longitude");
let latitude = row.get("Latitude");
let city_id = row.get("CityId");
let status_id = row.get("StatusId");


        Self {
            location_id,
abbreviation,
full_name,
address,
longitude,
latitude,
city_id,
status_id

        }
    }
}
