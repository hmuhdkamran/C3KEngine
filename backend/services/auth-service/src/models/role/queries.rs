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
pub struct Queries {
    pub query_id: Uuid,
pub full_name: String,
pub description: String,
pub status: i32

}

impl Queries {
    pub const TABLE: &'static str = r#""Role"."Queries""#;
    pub const PK: &'static str = "QueryId";
    pub const COLUMNS_ARRAY: [&'static str; 4] = ["QueryId","FullName","Description","Status"];

    pub fn get_id(&self) -> Uuid {
        self.query_id.clone()
    }

    pub fn get_args(&self) -> PgArguments {
        let mut args = PgArguments::default();
        let _ = args.add(self.query_id.clone());
let _ = args.add(self.full_name.clone());
let _ = args.add(self.description.clone());
let _ = args.add(self.status.clone());

        args
    }

    pub fn new(query_id: Uuid,full_name: String,description: String,status: i32) -> Self {
        Self {
            query_id,
full_name,
description,
status

        }
    }
}

impl PartialEq for Queries {
    fn eq(&self, other: &Self) -> bool {
        self.query_id == other.query_id
    }
}

impl Model for Queries {
    fn from_row(row: &PgRow) -> Queries
    where
        Self: Sized,
    {
        let query_id = row.get("QueryId");
let full_name = row.get("FullName");
let description = row.get("Description");
let status = row.get("Status");


        Self {
            query_id,
full_name,
description,
status

        }
    }
}
