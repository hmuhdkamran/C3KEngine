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
pub struct PaymentTerms {
    pub payment_term_id: Uuid,
pub abberviation: String,
pub full_name: String,
pub status_id: Uuid,
pub term_days: f64

}

impl PaymentTerms {
    pub const TABLE: &'static str = r#""PointOfSale"."PaymentTerms""#;
    pub const PK: &'static str = r#"PaymentTermId::TEXT=$1"#;
    pub const COLUMNS: &'static str = r#""PaymentTermId","Abberviation","FullName","StatusId","TermDays""#;
    pub const COLUMNS_UPDATE: &'static str = r#""PaymentTermId"=$1,"Abberviation"=$2,"FullName"=$3,"StatusId"=$4,"TermDays"=$5 WHERE "PaymentTermId"=$1"#;

    pub fn get_id(&self) -> Uuid {
        self.payment_term_id.clone()
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
