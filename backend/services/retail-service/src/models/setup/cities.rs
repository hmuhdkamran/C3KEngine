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
pub struct Cities {
    pub city_id: Uuid,
pub abbreviation: String,
pub full_name: String,
pub state_id: Uuid,
pub status_id: Uuid

}

impl Cities {
    pub const TABLE: &'static str = r#""Setup"."Cities""#;
    pub const PK: &'static str = r#"CityId::TEXT=$1"#;
    pub const COLUMNS: &'static str = r#""CityId","Abbreviation","FullName","StateId","StatusId""#;
    pub const COLUMNS_UPDATE: &'static str = r#""CityId"=$1,"Abbreviation"=$2,"FullName"=$3,"StateId"=$4,"StatusId"=$5 WHERE "CityId"=$1"#;

    pub fn get_id(&self) -> Uuid {
        self.city_id.clone()
    }

    pub fn new(city_id: Uuid,abbreviation: String,full_name: String,state_id: Uuid,status_id: Uuid) -> Self {
        Self {
            city_id,
abbreviation,
full_name,
state_id,
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
let abbreviation = row.get("Abbreviation");
let full_name = row.get("FullName");
let state_id = row.get("StateId");
let status_id = row.get("StatusId");


        Self {
            city_id,
abbreviation,
full_name,
state_id,
status_id

        }
    }
}
