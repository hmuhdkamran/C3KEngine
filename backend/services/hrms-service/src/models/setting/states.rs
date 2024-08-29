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
pub struct States {
    pub state_id: Uuid,
pub name: String,
pub country_id: Uuid,
pub status_id: Uuid

}

impl States {
    pub const TABLE: &'static str = r#""Setting"."States""#;
    pub const PK: &'static str = r#"StateId::TEXT=$1"#;
    pub const COLUMNS: &'static str = r#""StateId","Name","CountryId","StatusId""#;
    pub const COLUMNS_UPDATE: &'static str = r#""StateId"=$1,"Name"=$2,"CountryId"=$3,"StatusId"=$4 WHERE "StateId"=$1"#;

    pub fn get_id(&self) -> Uuid {
        self.state_id.clone()
    }

    pub fn new(state_id: Uuid,name: String,country_id: Uuid,status_id: Uuid) -> Self {
        Self {
            state_id,
name,
country_id,
status_id

        }
    }
}

impl PartialEq for States {
    fn eq(&self, other: &Self) -> bool {
        self.state_id == other.state_id
    }
}

impl Model for States {
    fn from_row(row: &PgRow) -> States
    where
        Self: Sized,
    {
        let state_id = row.get("StateId");
let name = row.get("Name");
let country_id = row.get("CountryId");
let status_id = row.get("StatusId");


        Self {
            state_id,
name,
country_id,
status_id

        }
    }
}
