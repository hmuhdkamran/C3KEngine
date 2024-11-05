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
pub struct Branches {
    pub branch_id: Uuid,
pub code: String,
pub name: String,
pub address: String,
pub location_id: Uuid,
pub status_id: Uuid

}

impl Branches {
    pub const TABLE: &'static str = r#""Setup"."Branches""#;
    pub const PK: &'static str = "BranchId";
    pub const COLUMNS_ARRAY: [&'static str; 6] = ["BranchId","Code","Name","Address","LocationId","StatusId"];

    pub fn get_id(&self) -> Uuid {
        self.branch_id.clone()
    }

    pub fn get_args(&self) -> PgArguments {
        let mut args = PgArguments::default();
        let _ = args.add(self.branch_id.clone());
let _ = args.add(self.code.clone());
let _ = args.add(self.name.clone());
let _ = args.add(self.address.clone());
let _ = args.add(self.location_id.clone());
let _ = args.add(self.status_id.clone());

        args
    }

    pub fn new(branch_id: Uuid,code: String,name: String,address: String,location_id: Uuid,status_id: Uuid) -> Self {
        Self {
            branch_id,
code,
name,
address,
location_id,
status_id

        }
    }
}

impl PartialEq for Branches {
    fn eq(&self, other: &Self) -> bool {
        self.branch_id == other.branch_id
    }
}

impl Model for Branches {
    fn from_row(row: &PgRow) -> Branches
    where
        Self: Sized,
    {
        let branch_id = row.get("BranchId");
let code = row.get("Code");
let name = row.get("Name");
let address = row.get("Address");
let location_id = row.get("LocationId");
let status_id = row.get("StatusId");


        Self {
            branch_id,
code,
name,
address,
location_id,
status_id

        }
    }
}
