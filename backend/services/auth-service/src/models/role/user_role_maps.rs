use c3k_common::interfaces::irepository::Model;
use serde::{Deserialize, Serialize};
pub use sqlx::{
    pool::PoolConnection,
    postgres::{PgArguments, PgPoolOptions, PgRow},
    types::chrono::{DateTime, Utc},
    Arguments, Error, PgPool, Postgres, Row,
};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct UserRoleMaps {
    pub user_role_map_id: Uuid,
    pub role_id: Uuid,
    pub user_id: Uuid,
    pub status_id: Uuid,
}

impl UserRoleMaps {
    pub const TABLE: &'static str = r#""Role"."UserRoleMaps""#;
    pub const PK: &'static str = "UserRoleMapId";
    pub const COLUMNS_ARRAY: [&'static str; 4] = ["UserRoleMapId", "RoleId", "UserId", "StatusId"];

    pub fn get_id(&self) -> Uuid {
        self.user_role_map_id.clone()
    }

    pub fn get_args(&self) -> PgArguments {
        let mut args = PgArguments::default();
        let _ = args.add(self.user_role_map_id.clone());
        let _ = args.add(self.role_id.clone());
        let _ = args.add(self.user_id.clone());
        let _ = args.add(self.status_id.clone());

        args
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

impl PartialEq for UserRoleMaps {
    fn eq(&self, other: &Self) -> bool {
        self.user_role_map_id == other.user_role_map_id
    }
}

impl Model for UserRoleMaps {
    fn from_row(row: &PgRow) -> UserRoleMaps
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
