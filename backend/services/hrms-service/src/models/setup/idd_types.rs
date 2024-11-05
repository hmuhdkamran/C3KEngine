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
pub struct IddTypes {
    pub idd_type_id: Uuid,
pub abbreviation: String,
pub full_name: String,
pub pattern: String,
pub status_id: Uuid

}

impl IddTypes {
    pub const TABLE: &'static str = r#""Setup"."IddTypes""#;
    pub const PK: &'static str = "IddTypeId";
    pub const COLUMNS_ARRAY: [&'static str; 5] = ["IddTypeId","Abbreviation","FullName","Pattern","StatusId"];

    pub fn get_id(&self) -> Uuid {
        self.idd_type_id.clone()
    }

    pub fn get_args(&self) -> PgArguments {
        let mut args = PgArguments::default();
        let _ = args.add(self.idd_type_id.clone());
let _ = args.add(self.abbreviation.clone());
let _ = args.add(self.full_name.clone());
let _ = args.add(self.pattern.clone());
let _ = args.add(self.status_id.clone());

        args
    }

    pub fn new(idd_type_id: Uuid,abbreviation: String,full_name: String,pattern: String,status_id: Uuid) -> Self {
        Self {
            idd_type_id,
abbreviation,
full_name,
pattern,
status_id

        }
    }
}

impl PartialEq for IddTypes {
    fn eq(&self, other: &Self) -> bool {
        self.idd_type_id == other.idd_type_id
    }
}

impl Model for IddTypes {
    fn from_row(row: &PgRow) -> IddTypes
    where
        Self: Sized,
    {
        let idd_type_id = row.get("IddTypeId");
let abbreviation = row.get("Abbreviation");
let full_name = row.get("FullName");
let pattern = row.get("Pattern");
let status_id = row.get("StatusId");


        Self {
            idd_type_id,
abbreviation,
full_name,
pattern,
status_id

        }
    }
}
