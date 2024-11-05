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
pub struct CustomerLoyaltyPoints {
    pub customer_loyalty_point_id: Uuid,
pub award_points: f64,
pub redeem_points: f64,
pub balance_points: f64,
pub bonus_points: f64

}

impl CustomerLoyaltyPoints {
    pub const TABLE: &'static str = r#""Customer"."CustomerLoyaltyPoints""#;
    pub const PK: &'static str = "CustomerLoyaltyPointId";
    pub const COLUMNS_ARRAY: [&'static str; 5] = ["CustomerLoyaltyPointId","AwardPoints","RedeemPoints","BalancePoints","BonusPoints"];

    pub fn get_id(&self) -> Uuid {
        self.customer_loyalty_point_id.clone()
    }

    pub fn get_args(&self) -> PgArguments {
        let mut args = PgArguments::default();
        let _ = args.add(self.customer_loyalty_point_id.clone());
let _ = args.add(self.award_points.clone());
let _ = args.add(self.redeem_points.clone());
let _ = args.add(self.balance_points.clone());
let _ = args.add(self.bonus_points.clone());

        args
    }

    pub fn new(customer_loyalty_point_id: Uuid,award_points: f64,redeem_points: f64,balance_points: f64,bonus_points: f64) -> Self {
        Self {
            customer_loyalty_point_id,
award_points,
redeem_points,
balance_points,
bonus_points

        }
    }
}

impl PartialEq for CustomerLoyaltyPoints {
    fn eq(&self, other: &Self) -> bool {
        self.customer_loyalty_point_id == other.customer_loyalty_point_id
    }
}

impl Model for CustomerLoyaltyPoints {
    fn from_row(row: &PgRow) -> CustomerLoyaltyPoints
    where
        Self: Sized,
    {
        let customer_loyalty_point_id = row.get("CustomerLoyaltyPointId");
let award_points = row.get("AwardPoints");
let redeem_points = row.get("RedeemPoints");
let balance_points = row.get("BalancePoints");
let bonus_points = row.get("BonusPoints");


        Self {
            customer_loyalty_point_id,
award_points,
redeem_points,
balance_points,
bonus_points

        }
    }
}
