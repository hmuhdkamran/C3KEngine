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
pub struct CandidateContacts {
    pub candidate_contact_id: Uuid,
pub candidate_id: Uuid,
pub contact_type_id: Uuid,
pub contact_value: String,
pub status_id: Uuid

}

impl CandidateContacts {
    pub const TABLE: &'static str = r#""Recruitment"."CandidateContacts""#;
    pub const PK: &'static str = "CandidateContactId";
    pub const COLUMNS_ARRAY: [&'static str; 5] = ["CandidateContactId","CandidateId","ContactTypeId","ContactValue","StatusId"];

    pub fn get_id(&self) -> Uuid {
        self.candidate_contact_id.clone()
    }

    pub fn get_args(&self) -> PgArguments {
        let mut args = PgArguments::default();
        let _ = args.add(self.candidate_contact_id.clone());
let _ = args.add(self.candidate_id.clone());
let _ = args.add(self.contact_type_id.clone());
let _ = args.add(self.contact_value.clone());
let _ = args.add(self.status_id.clone());

        args
    }

    pub fn new(candidate_contact_id: Uuid,candidate_id: Uuid,contact_type_id: Uuid,contact_value: String,status_id: Uuid) -> Self {
        Self {
            candidate_contact_id,
candidate_id,
contact_type_id,
contact_value,
status_id

        }
    }
}

impl PartialEq for CandidateContacts {
    fn eq(&self, other: &Self) -> bool {
        self.candidate_contact_id == other.candidate_contact_id
    }
}

impl Model for CandidateContacts {
    fn from_row(row: &PgRow) -> CandidateContacts
    where
        Self: Sized,
    {
        let candidate_contact_id = row.get("CandidateContactId");
let candidate_id = row.get("CandidateId");
let contact_type_id = row.get("ContactTypeId");
let contact_value = row.get("ContactValue");
let status_id = row.get("StatusId");


        Self {
            candidate_contact_id,
candidate_id,
contact_type_id,
contact_value,
status_id

        }
    }
}
