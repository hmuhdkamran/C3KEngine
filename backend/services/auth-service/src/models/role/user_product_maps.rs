use c3k_common::interfaces::irepository::Model;
use serde::{Deserialize, Serialize};
pub use sqlx::{
    pool::PoolConnection,
    postgres::{PgArguments, PgPoolOptions, PgRow},
    types::chrono::{DateTime, Utc},
    Arguments, Error, PgPool, Postgres, Row,
};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct UserProductMaps {
    pub user_product_map_id: Uuid,
    pub product_id: Uuid,
    pub user_id: Uuid,
    pub status_id: Uuid,
}

impl UserProductMaps {
    pub const TABLE: &'static str = r#""Role"."UserProductMaps""#;
    pub const PK: &'static str = "UserProductMapId";
    pub const COLUMNS_ARRAY: [&'static str; 4] =
        ["UserProductMapId", "ProductId", "UserId", "StatusId"];

    pub fn get_id(&self) -> Uuid {
        self.user_product_map_id.clone()
    }

    pub fn get_args(&self) -> PgArguments {
        let mut args = PgArguments::default();
        let _ = args.add(self.user_product_map_id.clone());
        let _ = args.add(self.product_id.clone());
        let _ = args.add(self.user_id.clone());
        let _ = args.add(self.status_id.clone());

        args
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

impl PartialEq for UserProductMaps {
    fn eq(&self, other: &Self) -> bool {
        self.user_product_map_id == other.user_product_map_id
    }
}

impl Model for UserProductMaps {
    fn from_row(row: &PgRow) -> UserProductMaps
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
