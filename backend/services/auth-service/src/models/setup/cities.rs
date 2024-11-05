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
pub struct Cities {
    pub city_id: Uuid,
pub country_id: Uuid,
pub abbreviation: String,
pub full_name: String,
pub status_id: Uuid

}

impl Cities {
    pub const TABLE: &'static str = r#""Setup"."Cities""#;
    pub const PK: &'static str = "CityId";
    pub const COLUMNS_ARRAY: [&'static str; 5] = ["CityId","CountryId","Abbreviation","FullName","StatusId"];

    pub fn get_id(&self) -> Uuid {
        self.city_id.clone()
    }

    pub fn get_args(&self) -> PgArguments {
        let mut args = PgArguments::default();
        let _ = args.add(self.city_id.clone());
let _ = args.add(self.country_id.clone());
let _ = args.add(self.abbreviation.clone());
let _ = args.add(self.full_name.clone());
let _ = args.add(self.status_id.clone());

        args
    }

    pub fn new(city_id: Uuid,country_id: Uuid,abbreviation: String,full_name: String,status_id: Uuid) -> Self {
        Self {
            city_id,
country_id,
abbreviation,
full_name,
status_id

        }
    }
}

impl PartialEq for Cities {
    fn eq(&self, other: &Self) -> bool {
        self.city_id == other.city_id
    }
}

impl Model for Cities {
    fn from_row(row: &PgRow) -> Cities
    where
        Self: Sized,
    {
        let city_id = row.get("CityId");
let country_id = row.get("CountryId");
let abbreviation = row.get("Abbreviation");
let full_name = row.get("FullName");
let status_id = row.get("StatusId");


        Self {
            city_id,
country_id,
abbreviation,
full_name,
status_id

        }
    }
}
