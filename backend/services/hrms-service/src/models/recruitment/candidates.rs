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
pub struct Candidates {
    pub candidate_id: Uuid,
pub fist_name: String,
pub middle_name: String,
pub last_name: String,
pub gender: String,
pub status_id: Uuid

}

impl Candidates {
    pub const TABLE: &'static str = r#""Recruitment"."Candidates""#;
    pub const PK: &'static str = "CandidateId";
    pub const COLUMNS_ARRAY: [&'static str; 6] = ["CandidateId","FistName","MiddleName","LastName","Gender","StatusId"];

    pub fn get_id(&self) -> Uuid {
        self.candidate_id.clone()
    }

    pub fn get_args(&self) -> PgArguments {
        let mut args = PgArguments::default();
        let _ = args.add(self.candidate_id.clone());
let _ = args.add(self.fist_name.clone());
let _ = args.add(self.middle_name.clone());
let _ = args.add(self.last_name.clone());
let _ = args.add(self.gender.clone());
let _ = args.add(self.status_id.clone());

        args
    }

    pub fn new(candidate_id: Uuid,fist_name: String,middle_name: String,last_name: String,gender: String,status_id: Uuid) -> Self {
        Self {
            candidate_id,
fist_name,
middle_name,
last_name,
gender,
status_id

        }
    }
}

impl PartialEq for Candidates {
    fn eq(&self, other: &Self) -> bool {
        self.candidate_id == other.candidate_id
    }
}

impl Model for Candidates {
    fn from_row(row: &PgRow) -> Candidates
    where
        Self: Sized,
    {
        let candidate_id = row.get("CandidateId");
let fist_name = row.get("FistName");
let middle_name = row.get("MiddleName");
let last_name = row.get("LastName");
let gender = row.get("Gender");
let status_id = row.get("StatusId");


        Self {
            candidate_id,
fist_name,
middle_name,
last_name,
gender,
status_id

        }
    }
}
