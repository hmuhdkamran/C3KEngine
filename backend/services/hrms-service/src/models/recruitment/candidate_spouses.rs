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
pub struct CandidateSpouses {
    pub candidate_spouse_id: Uuid,
pub candaidate_id: Uuid,
pub full_name: String,
pub contact_number: String,
pub spouse_type_id: Uuid,
pub status_id: Uuid

}

impl CandidateSpouses {
    pub const TABLE: &'static str = r#""Recruitment"."CandidateSpouses""#;
    pub const PK: &'static str = "CandidateSpouseId";
    pub const COLUMNS_ARRAY: [&'static str; 6] = ["CandidateSpouseId","CandaidateId","FullName","ContactNumber","SpouseTypeId","StatusId"];

    pub fn get_id(&self) -> Uuid {
        self.candidate_spouse_id.clone()
    }

    pub fn get_args(&self) -> PgArguments {
        let mut args = PgArguments::default();
        let _ = args.add(self.candidate_spouse_id.clone());
let _ = args.add(self.candaidate_id.clone());
let _ = args.add(self.full_name.clone());
let _ = args.add(self.contact_number.clone());
let _ = args.add(self.spouse_type_id.clone());
let _ = args.add(self.status_id.clone());

        args
    }

    pub fn new(candidate_spouse_id: Uuid,candaidate_id: Uuid,full_name: String,contact_number: String,spouse_type_id: Uuid,status_id: Uuid) -> Self {
        Self {
            candidate_spouse_id,
candaidate_id,
full_name,
contact_number,
spouse_type_id,
status_id

        }
    }
}

impl PartialEq for CandidateSpouses {
    fn eq(&self, other: &Self) -> bool {
        self.candidate_spouse_id == other.candidate_spouse_id
    }
}

impl Model for CandidateSpouses {
    fn from_row(row: &PgRow) -> CandidateSpouses
    where
        Self: Sized,
    {
        let candidate_spouse_id = row.get("CandidateSpouseId");
let candaidate_id = row.get("CandaidateId");
let full_name = row.get("FullName");
let contact_number = row.get("ContactNumber");
let spouse_type_id = row.get("SpouseTypeId");
let status_id = row.get("StatusId");


        Self {
            candidate_spouse_id,
candaidate_id,
full_name,
contact_number,
spouse_type_id,
status_id

        }
    }
}
