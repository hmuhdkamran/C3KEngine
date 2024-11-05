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
pub code: String,
pub name: String,
pub status_id: Uuid,
pub state_id: Uuid

}

impl Cities {
    pub const TABLE: &'static str = r#""Setting"."Cities""#;
    pub const PK: &'static str = "CityId";
    pub const COLUMNS_ARRAY: [&'static str; 5] = ["CityId","Code","Name","StatusId","StateId"];

    pub fn get_id(&self) -> Uuid {
        self.city_id.clone()
    }

    pub fn get_args(&self) -> PgArguments {
        let mut args = PgArguments::default();
        let _ = args.add(self.city_id.clone());
let _ = args.add(self.code.clone());
let _ = args.add(self.name.clone());
let _ = args.add(self.status_id.clone());
let _ = args.add(self.state_id.clone());

        args
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
