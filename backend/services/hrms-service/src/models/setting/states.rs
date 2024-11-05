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
pub struct States {
    pub state_id: Uuid,
pub name: String,
pub country_id: Uuid,
pub status_id: Uuid

}

impl States {
    pub const TABLE: &'static str = r#""Setting"."States""#;
    pub const PK: &'static str = "StateId";
    pub const COLUMNS_ARRAY: [&'static str; 4] = ["StateId","Name","CountryId","StatusId"];

    pub fn get_id(&self) -> Uuid {
        self.state_id.clone()
    }

    pub fn get_args(&self) -> PgArguments {
        let mut args = PgArguments::default();
        let _ = args.add(self.state_id.clone());
let _ = args.add(self.name.clone());
let _ = args.add(self.country_id.clone());
let _ = args.add(self.status_id.clone());

        args
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
