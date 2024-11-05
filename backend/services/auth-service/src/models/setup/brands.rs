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
pub struct Brands {
    pub brand_id: Uuid,
pub business_id: Uuid,
pub abbreviation: String,
pub full_name: String,
pub tag_line: String,
pub logo: String,
pub dated: DateTime<Utc>,
pub status_id: Uuid

}

impl Brands {
    pub const TABLE: &'static str = r#""Setup"."Brands""#;
    pub const PK: &'static str = "BrandId";
    pub const COLUMNS_ARRAY: [&'static str; 8] = ["BrandId","BusinessId","Abbreviation","FullName","TagLine","Logo","Dated","StatusId"];

    pub fn get_id(&self) -> Uuid {
        self.brand_id.clone()
    }

    pub fn get_args(&self) -> PgArguments {
        let mut args = PgArguments::default();
        let _ = args.add(self.brand_id.clone());
let _ = args.add(self.business_id.clone());
let _ = args.add(self.abbreviation.clone());
let _ = args.add(self.full_name.clone());
let _ = args.add(self.tag_line.clone());
let _ = args.add(self.logo.clone());
let _ = args.add(self.dated.clone());
let _ = args.add(self.status_id.clone());

        args
    }

    pub fn new(brand_id: Uuid,business_id: Uuid,abbreviation: String,full_name: String,tag_line: String,logo: String,dated: DateTime<Utc>,status_id: Uuid) -> Self {
        Self {
            brand_id,
business_id,
abbreviation,
full_name,
tag_line,
logo,
dated,
status_id

        }
    }
}

impl PartialEq for Brands {
    fn eq(&self, other: &Self) -> bool {
        self.brand_id == other.brand_id
    }
}

impl Model for Brands {
    fn from_row(row: &PgRow) -> Brands
    where
        Self: Sized,
    {
        let brand_id = row.get("BrandId");
let business_id = row.get("BusinessId");
let abbreviation = row.get("Abbreviation");
let full_name = row.get("FullName");
let tag_line = row.get("TagLine");
let logo = row.get("Logo");
let dated = row.get("Dated");
let status_id = row.get("StatusId");


        Self {
            brand_id,
business_id,
abbreviation,
full_name,
tag_line,
logo,
dated,
status_id

        }
    }
}
