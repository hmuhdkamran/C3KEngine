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
pub struct Skills {
    pub skill_id: Uuid,
pub full_name: String,
pub status_id: Uuid,
pub abbreviation: String

}

impl Skills {
    pub const TABLE: &'static str = r#""Setup"."Skills""#;
    pub const PK: &'static str = "SkillId";
    pub const COLUMNS_ARRAY: [&'static str; 4] = ["SkillId","FullName","StatusId","Abbreviation"];

    pub fn get_id(&self) -> Uuid {
        self.skill_id.clone()
    }

    pub fn get_args(&self) -> PgArguments {
        let mut args = PgArguments::default();
        let _ = args.add(self.skill_id.clone());
let _ = args.add(self.full_name.clone());
let _ = args.add(self.status_id.clone());
let _ = args.add(self.abbreviation.clone());

        args
    }

    pub fn new(skill_id: Uuid,full_name: String,status_id: Uuid,abbreviation: String) -> Self {
        Self {
            skill_id,
full_name,
status_id,
abbreviation

        }
    }
}

impl PartialEq for Skills {
    fn eq(&self, other: &Self) -> bool {
        self.skill_id == other.skill_id
    }
}

impl Model for Skills {
    fn from_row(row: &PgRow) -> Skills
    where
        Self: Sized,
    {
        let skill_id = row.get("SkillId");
let full_name = row.get("FullName");
let status_id = row.get("StatusId");
let abbreviation = row.get("Abbreviation");


        Self {
            skill_id,
full_name,
status_id,
abbreviation

        }
    }
}
