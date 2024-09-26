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
    pub const PK: &'static str = r#"BranchId::TEXT=$1"#;
    pub const COLUMNS: &'static str = r#""BranchId","Code","Name","Address","LocationId","StatusId""#;
    pub const COLUMNS_UPDATE: &'static str = r#""BranchId"=$1,"Code"=$2,"Name"=$3,"Address"=$4,"LocationId"=$5,"StatusId"=$6 WHERE "BranchId"=$1"#;

    pub fn get_id(&self) -> Uuid {
        self.branch_id.clone()
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
