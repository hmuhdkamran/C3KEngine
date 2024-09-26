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
pub struct CampaignProducts {
    pub campaign_product_id: Uuid,
pub campaign_id: Uuid,
pub product_id: Uuid,
pub discount_type_id: Uuid,
pub discount_value: f64,
pub status_id: Uuid

}

impl CampaignProducts {
    pub const TABLE: &'static str = r#""PointOfSale"."CampaignProducts""#;
    pub const PK: &'static str = r#"CampaignProductId::TEXT=$1"#;
    pub const COLUMNS: &'static str = r#""CampaignProductId","CampaignId","ProductId","DiscountTypeId","DiscountValue","StatusId""#;
    pub const COLUMNS_UPDATE: &'static str = r#""CampaignProductId"=$1,"CampaignId"=$2,"ProductId"=$3,"DiscountTypeId"=$4,"DiscountValue"=$5,"StatusId"=$6 WHERE "CampaignProductId"=$1"#;

    pub fn get_id(&self) -> Uuid {
        self.campaign_product_id.clone()
    }

    pub fn new(campaign_product_id: Uuid,campaign_id: Uuid,product_id: Uuid,discount_type_id: Uuid,discount_value: f64,status_id: Uuid) -> Self {
        Self {
            campaign_product_id,
campaign_id,
product_id,
discount_type_id,
discount_value,
status_id

        }
    }
}

impl PartialEq for CampaignProducts {
    fn eq(&self, other: &Self) -> bool {
        self.campaign_product_id == other.campaign_product_id
    }
}

impl Model for CampaignProducts {
    fn from_row(row: &PgRow) -> CampaignProducts
    where
        Self: Sized,
    {
        let campaign_product_id = row.get("CampaignProductId");
let campaign_id = row.get("CampaignId");
let product_id = row.get("ProductId");
let discount_type_id = row.get("DiscountTypeId");
let discount_value = row.get("DiscountValue");
let status_id = row.get("StatusId");


        Self {
            campaign_product_id,
campaign_id,
product_id,
discount_type_id,
discount_value,
status_id

        }
    }
}
