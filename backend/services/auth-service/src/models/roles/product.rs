use c3k_common::interfaces::irepository::Model;
use serde::{Deserialize, Serialize};
pub use sqlx::{
    pool::PoolConnection,
    postgres::{PgArguments, PgPoolOptions, PgRow},
    Arguments, Error, PgPool, Postgres, Row,
};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Product {
    pub product_id: Uuid,
    pub abbreviation: String,
    pub full_name: String,
    pub description: String,
    pub icon: String,
    pub status_id: Uuid,
}

impl Product {
    pub const TABLE: &'static str = r#""Role"."Products""#;
    pub const PK: &'static str = r#""ProductId"::TEXT=$1"#;
    pub const COLUMNS: &'static str =
        r#""ProductId", "Abbreviation", "FullName", "Description", "Icon", "StatusId""#;
    pub const COLUMNS_UPDATE: &'static str = r#""Abbreviation"=$2, "Description"=$3, "Icon"=$4, "Password"=$5, "StatusId"=$6 WHERE "ProductId"=$1"#;

    pub fn get_id(&self) -> Uuid {
        self.product_id.clone()
    }

    pub fn new(
        product_id: Uuid,
        abbreviation: String,
        full_name: String,
        description: String,
        icon: String,
        status_id: Uuid,
    ) -> Self {
        Self {
            product_id,
            abbreviation,
            full_name,
            description,
            icon,
            status_id,
        }
    }
}

impl PartialEq for Product {
    fn eq(&self, other: &Self) -> bool {
        self.product_id == other.product_id
    }
}

impl Model for Product {
    fn from_row(row: &PgRow) -> Product
    where
        Self: Sized,
    {
        let product_id = row.get("ProductId");
        let abbreviation = row.get("Abbreviation");
        let full_name = row.get("FullName");
        let description = row.get("Description");
        let icon = row.get("Icon");
        let status_id = row.get("StatusId");

        Self {
            product_id,
            abbreviation,
            full_name,
            description,
            icon,
            status_id,
        }
    }
}
