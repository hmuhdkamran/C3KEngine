use c3k_common::interfaces::irepository::Model;
use serde::{Deserialize, Serialize};
pub use sqlx::{
    pool::PoolConnection,
    postgres::{PgArguments, PgPoolOptions, PgRow},
    Arguments, Error, PgPool, Postgres, Row,
};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Role {
    pub role_id: Uuid,
    pub parent_role_id: Uuid,
    pub full_name: String,
    pub status_id: Uuid,
}

impl Role {
    pub const TABLE: &'static str = r#""Role"."Roles""#;
    pub const PK: &'static str = r#""RoleId"::TEXT=$1"#;
    pub const COLUMNS: &'static str = r#""RoleId", "ParentRoleId", "FullName", "StatusId""#;
    pub const COLUMNS_UPDATE: &'static str =
        r#""ParentRoleId"=$2, "FullName"=$3, "StatusId"=$4 WHERE "RoleId"=$1"#;

    pub fn get_id(&self) -> Uuid {
        self.role_id.clone()
    }

    pub fn new(role_id: Uuid, parent_role_id: Uuid, full_name: String, status_id: Uuid) -> Self {
        Self {
            role_id,
            parent_role_id,
            full_name,
            status_id,
        }
    }
}

impl PartialEq for Role {
    fn eq(&self, other: &Self) -> bool {
        self.role_id == other.role_id
    }
}

impl Model for Role {
    fn from_row(row: &PgRow) -> Role
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
            status_id,
        }
    }
}
