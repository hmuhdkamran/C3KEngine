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
pub struct UserProductMap {
    pub user_product_map_id: Uuid,
    pub product_id: Uuid,
    pub user_id: Uuid,
    pub status_id: Uuid,
}

impl UserProductMap {
    pub const TABLE: &'static str = r#""Role"."UserProductMap""#;
    pub const PK: &'static str = r#""UserProductMapId"::TEXT=$1"#;
    pub const COLUMNS: &'static str = r#""UserProductMapId", "ProductId", "UserId", "StatusId""#;
    pub const COLUMNS_UPDATE: &'static str =
        r#""ProductId"=$2, "UserId"=$3, "StatusId"=$4 WHERE "UserProductMapId"=$1"#;

    pub fn get_id(&self) -> Uuid {
        self.user_product_map_id.clone()
    }

    pub fn new(
        user_product_map_id: Uuid,
        product_id: Uuid,
        user_id: Uuid,
        status_id: Uuid,
    ) -> Self {
        Self {
            user_product_map_id,
            product_id,
            user_id,
            status_id,
        }
    }
}

impl PartialEq for UserProductMap {
    fn eq(&self, other: &Self) -> bool {
        self.user_product_map_id == other.user_product_map_id
    }
}

impl Model for UserProductMap {
    fn from_row(row: &PgRow) -> UserProductMap
    where
        Self: Sized,
    {
        let user_product_map_id = row.get("UserProductMapId");
        let product_id = row.get("ProductId");
        let user_id = row.get("UserId");
        let status_id = row.get("StatusId");

        Self {
            user_product_map_id,
            product_id,
            user_id,
            status_id,
        }
    }
}
