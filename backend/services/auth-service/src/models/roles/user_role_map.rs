use serde::{Deserialize, Serialize};
pub use sqlx::{
    pool::PoolConnection,
    postgres::{PgArguments, PgPoolOptions, PgRow},
    Arguments, Error, PgPool, Postgres, Row,
};
use uuid::Uuid;
use crate::interfaces::irepository::Model;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserRoleMap {
    pub user_role_map_id: Uuid,
    pub role_id: Uuid,
    pub user_id: Uuid,
    pub status_id: Uuid,
}

impl UserRoleMap {
    pub const TABLE: &'static str = r#""Role"."UserRoleMaps""#;
    pub const PK: &'static str = r#""UserRoleMapId"::TEXT=$1"#;
    pub const COLUMNS: &'static str = r#""UserRoleMapId", "RoleId", "UserId", "StatusId""#;
    pub const COLUMNS_UPDATE: &'static str =
        r#""RoleId"=$2, "UserId"=$3, "StatusId"=$4 WHERE "UserRoleMapId"=$1"#;

    pub fn get_id(&self) -> Uuid {
        self.user_role_map_id.clone()
    }

    pub fn new(user_role_map_id: Uuid, role_id: Uuid, user_id: Uuid, status_id: Uuid) -> Self {
        Self {
            user_role_map_id,
            role_id,
            user_id,
            status_id,
        }
    }
}

impl PartialEq for UserRoleMap {
    fn eq(&self, other: &Self) -> bool {
        self.user_role_map_id == other.user_role_map_id
    }
}

impl Model for UserRoleMap {
    fn from_row(row: &PgRow) -> UserRoleMap
    where
        Self: Sized,
    {
        let user_role_map_id = row.get("UserRoleMapId");
        let role_id = row.get("RoleId");
        let user_id = row.get("UserId");
        let status_id = row.get("StatusId");

        Self {
            user_role_map_id,
            role_id,
            user_id,
            status_id,
        }
    }
}
