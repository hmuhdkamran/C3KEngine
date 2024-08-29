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
pub struct Relations {
    pub relation_id: Uuid,
pub full_name: String,
pub status_id: Uuid,
pub abbreviation: String

}

impl Relations {
    pub const TABLE: &'static str = r#""Setup"."Relations""#;
    pub const PK: &'static str = r#"RelationId::TEXT=$1"#;
    pub const COLUMNS: &'static str = r#""RelationId","FullName","StatusId","Abbreviation""#;
    pub const COLUMNS_UPDATE: &'static str = r#""RelationId"=$1,"FullName"=$2,"StatusId"=$3,"Abbreviation"=$4 WHERE "RelationId"=$1"#;

    pub fn get_id(&self) -> Uuid {
        self.relation_id.clone()
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
