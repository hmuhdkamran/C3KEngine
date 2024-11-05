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
pub struct RoleRouteMaps {
    pub role_route_map_id: Uuid,
pub role_id: Uuid,
pub route_id: Uuid,
pub operation: String,
pub status_id: Uuid

}

impl RoleRouteMaps {
    pub const TABLE: &'static str = r#""Role"."RoleRouteMaps""#;
    pub const PK: &'static str = "RoleRouteMapId";
    pub const COLUMNS_ARRAY: [&'static str; 5] = ["RoleRouteMapId","RoleId","RouteId","Operation","StatusId"];

    pub fn get_id(&self) -> Uuid {
        self.role_route_map_id.clone()
    }

    pub fn get_args(&self) -> PgArguments {
        let mut args = PgArguments::default();
        let _ = args.add(self.role_route_map_id.clone());
let _ = args.add(self.role_id.clone());
let _ = args.add(self.route_id.clone());
let _ = args.add(self.operation.clone());
let _ = args.add(self.status_id.clone());

        args
    }

    pub fn new(role_route_map_id: Uuid,role_id: Uuid,route_id: Uuid,operation: String,status_id: Uuid) -> Self {
        Self {
            role_route_map_id,
role_id,
route_id,
operation,
status_id

        }
    }
}

impl PartialEq for RoleRouteMaps {
    fn eq(&self, other: &Self) -> bool {
        self.role_route_map_id == other.role_route_map_id
    }
}

impl Model for RoleRouteMaps {
    fn from_row(row: &PgRow) -> RoleRouteMaps
    where
        Self: Sized,
    {
        let role_route_map_id = row.get("RoleRouteMapId");
let role_id = row.get("RoleId");
let route_id = row.get("RouteId");
let operation = row.get("Operation");
let status_id = row.get("StatusId");


        Self {
            role_route_map_id,
role_id,
route_id,
operation,
status_id

        }
    }
}
