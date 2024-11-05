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
pub struct Socials {
    pub social_id: Uuid,
pub abbreviation: String,
pub full_name: String,
pub icon: String,
pub status_id: Uuid

}

impl Socials {
    pub const TABLE: &'static str = r#""Setup"."Socials""#;
    pub const PK: &'static str = "SocialId";
    pub const COLUMNS_ARRAY: [&'static str; 5] = ["SocialId","Abbreviation","FullName","Icon","StatusId"];

    pub fn get_id(&self) -> Uuid {
        self.social_id.clone()
    }

    pub fn get_args(&self) -> PgArguments {
        let mut args = PgArguments::default();
        let _ = args.add(self.social_id.clone());
let _ = args.add(self.abbreviation.clone());
let _ = args.add(self.full_name.clone());
let _ = args.add(self.icon.clone());
let _ = args.add(self.status_id.clone());

        args
    }

    pub fn new(social_id: Uuid,abbreviation: String,full_name: String,icon: String,status_id: Uuid) -> Self {
        Self {
            social_id,
abbreviation,
full_name,
icon,
status_id

        }
    }
}

impl PartialEq for Socials {
    fn eq(&self, other: &Self) -> bool {
        self.social_id == other.social_id
    }
}

impl Model for Socials {
    fn from_row(row: &PgRow) -> Socials
    where
        Self: Sized,
    {
        let social_id = row.get("SocialId");
let abbreviation = row.get("Abbreviation");
let full_name = row.get("FullName");
let icon = row.get("Icon");
let status_id = row.get("StatusId");


        Self {
            social_id,
abbreviation,
full_name,
icon,
status_id

        }
    }
}
