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
pub struct Deductions {
    pub deduction_id: Uuid,
pub abbreviation: String,
pub full_name: String,
pub status_id: Uuid

}

impl Deductions {
    pub const TABLE: &'static str = r#""Payroll"."Deductions""#;
    pub const PK: &'static str = "DeductionId";
    pub const COLUMNS_ARRAY: [&'static str; 4] = ["DeductionId","Abbreviation","FullName","StatusId"];

    pub fn get_id(&self) -> Uuid {
        self.deduction_id.clone()
    }

    pub fn get_args(&self) -> PgArguments {
        let mut args = PgArguments::default();
        let _ = args.add(self.deduction_id.clone());
let _ = args.add(self.abbreviation.clone());
let _ = args.add(self.full_name.clone());
let _ = args.add(self.status_id.clone());

        args
    }

    pub fn new(deduction_id: Uuid,abbreviation: String,full_name: String,status_id: Uuid) -> Self {
        Self {
            deduction_id,
abbreviation,
full_name,
status_id

        }
    }
}

impl PartialEq for Deductions {
    fn eq(&self, other: &Self) -> bool {
        self.deduction_id == other.deduction_id
    }
}

impl Model for Deductions {
    fn from_row(row: &PgRow) -> Deductions
    where
        Self: Sized,
    {
        let deduction_id = row.get("DeductionId");
let abbreviation = row.get("Abbreviation");
let full_name = row.get("FullName");
let status_id = row.get("StatusId");


        Self {
            deduction_id,
abbreviation,
full_name,
status_id

        }
    }
}
