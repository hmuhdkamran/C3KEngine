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
pub struct PaymentTerms {
    pub payment_term_id: Uuid,
pub abberviation: String,
pub full_name: String,
pub status_id: Uuid,
pub term_days: f64

}

impl PaymentTerms {
    pub const TABLE: &'static str = r#""PointOfSale"."PaymentTerms""#;
    pub const PK: &'static str = "PaymentTermId";
    pub const COLUMNS_ARRAY: [&'static str; 5] = ["PaymentTermId","Abberviation","FullName","StatusId","TermDays"];

    pub fn get_id(&self) -> Uuid {
        self.payment_term_id.clone()
    }

    pub fn get_args(&self) -> PgArguments {
        let mut args = PgArguments::default();
        let _ = args.add(self.payment_term_id.clone());
let _ = args.add(self.abberviation.clone());
let _ = args.add(self.full_name.clone());
let _ = args.add(self.status_id.clone());
let _ = args.add(self.term_days.clone());

        args
    }

    pub fn new(payment_term_id: Uuid,abberviation: String,full_name: String,status_id: Uuid,term_days: f64) -> Self {
        Self {
            payment_term_id,
abberviation,
full_name,
status_id,
term_days

        }
    }
}

impl PartialEq for PaymentTerms {
    fn eq(&self, other: &Self) -> bool {
        self.payment_term_id == other.payment_term_id
    }
}

impl Model for PaymentTerms {
    fn from_row(row: &PgRow) -> PaymentTerms
    where
        Self: Sized,
    {
        let payment_term_id = row.get("PaymentTermId");
let abberviation = row.get("Abberviation");
let full_name = row.get("FullName");
let status_id = row.get("StatusId");
let term_days = row.get("TermDays");


        Self {
            payment_term_id,
abberviation,
full_name,
status_id,
term_days

        }
    }
}
