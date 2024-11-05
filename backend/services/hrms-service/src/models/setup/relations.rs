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
pub struct Relations {
    pub relation_id: Uuid,
pub full_name: String,
pub status_id: Uuid,
pub abbreviation: String

}

impl Relations {
    pub const TABLE: &'static str = r#""Setup"."Relations""#;
    pub const PK: &'static str = "RelationId";
    pub const COLUMNS_ARRAY: [&'static str; 4] = ["RelationId","FullName","StatusId","Abbreviation"];

    pub fn get_id(&self) -> Uuid {
        self.relation_id.clone()
    }

    pub fn get_args(&self) -> PgArguments {
        let mut args = PgArguments::default();
        let _ = args.add(self.relation_id.clone());
let _ = args.add(self.full_name.clone());
let _ = args.add(self.status_id.clone());
let _ = args.add(self.abbreviation.clone());

        args
    }

    pub fn new(relation_id: Uuid,full_name: String,status_id: Uuid,abbreviation: String) -> Self {
        Self {
            relation_id,
full_name,
status_id,
abbreviation

        }
    }
}

impl PartialEq for Relations {
    fn eq(&self, other: &Self) -> bool {
        self.relation_id == other.relation_id
    }
}

impl Model for Relations {
    fn from_row(row: &PgRow) -> Relations
    where
        Self: Sized,
    {
        let relation_id = row.get("RelationId");
let full_name = row.get("FullName");
let status_id = row.get("StatusId");
let abbreviation = row.get("Abbreviation");


        Self {
            relation_id,
full_name,
status_id,
abbreviation

        }
    }
}
