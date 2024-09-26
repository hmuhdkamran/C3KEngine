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
pub struct Campaigns {
    pub campaign_id: Uuid,
pub fulle_name: String,
pub start_date: DateTime<Utc>,
pub end_date: DateTime<Utc>,
pub status_id: Uuid

}

impl Campaigns {
    pub const TABLE: &'static str = r#""PointOfSale"."Campaigns""#;
    pub const PK: &'static str = r#"CampaignId::TEXT=$1"#;
    pub const COLUMNS: &'static str = r#""CampaignId","FulleName","StartDate","EndDate","StatusId""#;
    pub const COLUMNS_UPDATE: &'static str = r#""CampaignId"=$1,"FulleName"=$2,"StartDate"=$3,"EndDate"=$4,"StatusId"=$5 WHERE "CampaignId"=$1"#;

    pub fn get_id(&self) -> Uuid {
        self.campaign_id.clone()
    }

    pub fn new(campaign_id: Uuid,fulle_name: String,start_date: DateTime<Utc>,end_date: DateTime<Utc>,status_id: Uuid) -> Self {
        Self {
            campaign_id,
fulle_name,
start_date,
end_date,
status_id

        }
    }
}

impl PartialEq for Campaigns {
    fn eq(&self, other: &Self) -> bool {
        self.campaign_id == other.campaign_id
    }
}

impl Model for Campaigns {
    fn from_row(row: &PgRow) -> Campaigns
    where
        Self: Sized,
    {
        let campaign_id = row.get("CampaignId");
let fulle_name = row.get("FulleName");
let start_date = row.get("StartDate");
let end_date = row.get("EndDate");
let status_id = row.get("StatusId");


        Self {
            campaign_id,
fulle_name,
start_date,
end_date,
status_id

        }
    }
}
