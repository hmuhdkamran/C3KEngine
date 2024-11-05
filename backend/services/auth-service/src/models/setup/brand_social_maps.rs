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
pub struct BrandSocialMaps {
    pub brand_social_map_id: Uuid,
pub social_id: Uuid,
pub link: String,
pub status_id: Uuid

}

impl BrandSocialMaps {
    pub const TABLE: &'static str = r#""Setup"."BrandSocialMaps""#;
    pub const PK: &'static str = "BrandSocialMapId";
    pub const COLUMNS_ARRAY: [&'static str; 4] = ["BrandSocialMapId","SocialId","Link","StatusId"];

    pub fn get_id(&self) -> Uuid {
        self.brand_social_map_id.clone()
    }

    pub fn get_args(&self) -> PgArguments {
        let mut args = PgArguments::default();
        let _ = args.add(self.brand_social_map_id.clone());
let _ = args.add(self.social_id.clone());
let _ = args.add(self.link.clone());
let _ = args.add(self.status_id.clone());

        args
    }

    pub fn new(brand_social_map_id: Uuid,social_id: Uuid,link: String,status_id: Uuid) -> Self {
        Self {
            brand_social_map_id,
social_id,
link,
status_id

        }
    }
}

impl PartialEq for BrandSocialMaps {
    fn eq(&self, other: &Self) -> bool {
        self.brand_social_map_id == other.brand_social_map_id
    }
}

impl Model for BrandSocialMaps {
    fn from_row(row: &PgRow) -> BrandSocialMaps
    where
        Self: Sized,
    {
        let brand_social_map_id = row.get("BrandSocialMapId");
let social_id = row.get("SocialId");
let link = row.get("Link");
let status_id = row.get("StatusId");


        Self {
            brand_social_map_id,
social_id,
link,
status_id

        }
    }
}
