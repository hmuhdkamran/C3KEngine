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
pub struct BloodGroups {
    pub blood_group_id: Uuid,
pub full_name: String,
pub status_id: Uuid,
pub abbreviation: String

}

impl BloodGroups {
    pub const TABLE: &'static str = r#""Setup"."BloodGroups""#;
    pub const PK: &'static str = "BloodGroupId";
    pub const COLUMNS_ARRAY: [&'static str; 4] = ["BloodGroupId","FullName","StatusId","Abbreviation"];

    pub fn get_id(&self) -> Uuid {
        self.blood_group_id.clone()
    }

    pub fn get_args(&self) -> PgArguments {
        let mut args = PgArguments::default();
        let _ = args.add(self.blood_group_id.clone());
let _ = args.add(self.full_name.clone());
let _ = args.add(self.status_id.clone());
let _ = args.add(self.abbreviation.clone());

        args
    }

    pub fn new(blood_group_id: Uuid,full_name: String,status_id: Uuid,abbreviation: String) -> Self {
        Self {
            blood_group_id,
full_name,
status_id,
abbreviation

        }
    }
}

impl PartialEq for BloodGroups {
    fn eq(&self, other: &Self) -> bool {
        self.blood_group_id == other.blood_group_id
    }
}

impl Model for BloodGroups {
    fn from_row(row: &PgRow) -> BloodGroups
    where
        Self: Sized,
    {
        let blood_group_id = row.get("BloodGroupId");
let full_name = row.get("FullName");
let status_id = row.get("StatusId");
let abbreviation = row.get("Abbreviation");


        Self {
            blood_group_id,
full_name,
status_id,
abbreviation

        }
    }
}
