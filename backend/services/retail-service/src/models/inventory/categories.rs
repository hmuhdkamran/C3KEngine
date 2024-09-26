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
pub struct Categories {
    pub category_id: Uuid,
pub abbreviation: String,
pub full_name: String,
pub parent_category_id: Uuid,
pub status_id: Uuid

}

impl Categories {
    pub const TABLE: &'static str = r#""Inventory"."Categories""#;
    pub const PK: &'static str = r#"CategoryId::TEXT=$1"#;
    pub const COLUMNS: &'static str = r#""CategoryId","Abbreviation","FullName","ParentCategoryId","StatusId""#;
    pub const COLUMNS_UPDATE: &'static str = r#""CategoryId"=$1,"Abbreviation"=$2,"FullName"=$3,"ParentCategoryId"=$4,"StatusId"=$5 WHERE "CategoryId"=$1"#;

    pub fn get_id(&self) -> Uuid {
        self.category_id.clone()
    }

    pub fn new(category_id: Uuid,abbreviation: String,full_name: String,parent_category_id: Uuid,status_id: Uuid) -> Self {
        Self {
            category_id,
abbreviation,
full_name,
parent_category_id,
status_id

        }
    }
}

impl PartialEq for Categories {
    fn eq(&self, other: &Self) -> bool {
        self.category_id == other.category_id
    }
}

impl Model for Categories {
    fn from_row(row: &PgRow) -> Categories
    where
        Self: Sized,
    {
        let category_id = row.get("CategoryId");
let abbreviation = row.get("Abbreviation");
let full_name = row.get("FullName");
let parent_category_id = row.get("ParentCategoryId");
let status_id = row.get("StatusId");


        Self {
            category_id,
abbreviation,
full_name,
parent_category_id,
status_id

        }
    }
}
