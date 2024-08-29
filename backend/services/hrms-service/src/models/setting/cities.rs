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
pub code: String,
pub name: String,
pub status_id: Uuid,
pub state_id: Uuid

}

impl Cities {
    pub const TABLE: &'static str = r#""Setting"."Cities""#;
    pub const PK: &'static str = r#"CityId::TEXT=$1"#;
    pub const COLUMNS: &'static str = r#""CityId","Code","Name","StatusId","StateId""#;
    pub const COLUMNS_UPDATE: &'static str = r#""CityId"=$1,"Code"=$2,"Name"=$3,"StatusId"=$4,"StateId"=$5 WHERE "CityId"=$1"#;

    pub fn get_id(&self) -> Uuid {
        self.city_id.clone()
    }

    pub fn new(city_id: Uuid,code: String,name: String,status_id: Uuid,state_id: Uuid) -> Self {
        Self {
            city_id,
code,
name,
status_id,
state_id

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
let code = row.get("Code");
let name = row.get("Name");
let status_id = row.get("StatusId");
let state_id = row.get("StateId");


        Self {
            city_id,
code,
name,
status_id,
state_id

        }
    }
}
