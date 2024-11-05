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
pub struct Roles {
    pub role_id: Uuid,
pub parent_role_id: Uuid,
pub full_name: String,
pub status_id: Uuid

}

impl Roles {
    pub const TABLE: &'static str = r#""Role"."Roles""#;
    pub const PK: &'static str = "RoleId";
    pub const COLUMNS_ARRAY: [&'static str; 4] = ["RoleId","ParentRoleId","FullName","StatusId"];

    pub fn get_id(&self) -> Uuid {
        self.role_id.clone()
    }

    pub fn get_args(&self) -> PgArguments {
        let mut args = PgArguments::default();
        let _ = args.add(self.role_id.clone());
let _ = args.add(self.parent_role_id.clone());
let _ = args.add(self.full_name.clone());
let _ = args.add(self.status_id.clone());

        args
    }

    pub fn new(role_id: Uuid,parent_role_id: Uuid,full_name: String,status_id: Uuid) -> Self {
        Self {
            role_id,
parent_role_id,
full_name,
status_id

        }
    }
}

impl PartialEq for Roles {
    fn eq(&self, other: &Self) -> bool {
        self.role_id == other.role_id
    }
}

impl Model for Roles {
    fn from_row(row: &PgRow) -> Roles
    where
        Self: Sized,
    {
        let role_id = row.get("RoleId");
let parent_role_id = row.get("ParentRoleId");
let full_name = row.get("FullName");
let status_id = row.get("StatusId");


        Self {
            role_id,
parent_role_id,
full_name,
status_id

        }
    }
}
