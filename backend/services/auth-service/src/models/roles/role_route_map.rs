use c3k_common::interfaces::irepository::Model;
use serde::{Deserialize, Serialize};
pub use sqlx::{
    pool::PoolConnection,
    postgres::{PgArguments, PgPoolOptions, PgRow},
    Arguments, Error, PgPool, Postgres, Row,
};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoleRouteMap {
    pub role_route_id: Uuid,
    pub role_id: Uuid,
    pub route_id: Uuid,
    pub operation: String,
    pub status_id: Uuid,
}

impl RoleRouteMap {
    pub const TABLE: &'static str = r#""Role"."RoleRouteMaps""#;
    pub const PK: &'static str = r#""RoleRouteMapId"::TEXT=$1"#;
    pub const COLUMNS: &'static str =
        r#""RoleRouteMapId", "RoleId", "RouteId", "Operation", "StatusId""#;
    pub const COLUMNS_UPDATE: &'static str =
        r#""RoleId"=$2, "RouteId"=$3, "Operation"=$4, "StatusId"=$5 WHERE "RoleRouteMapId"=$1"#;

    pub fn get_id(&self) -> Uuid {
        self.route_id.clone()
    }

    pub fn new(
        role_route_id: Uuid,
        role_id: Uuid,
        route_id: Uuid,
        operation: String,
        status_id: Uuid,
    ) -> Self {
        Self {
            role_route_id,
            role_id,
            route_id,
            operation,
            status_id,
        }
    }
}

impl PartialEq for RoleRouteMap {
    fn eq(&self, other: &Self) -> bool {
        self.route_id == other.route_id
    }
}

impl Model for RoleRouteMap {
    fn from_row(row: &PgRow) -> RoleRouteMap
    where
        Self: Sized,
    {
        let role_route_id = row.get("RoleRouteMapId");
        let role_id = row.get("RoleId");
        let route_id = row.get("RouteId");
        let operation = row.get("Operation");
        let status_id = row.get("StatusId");

        Self {
            role_route_id,
            role_id,
            route_id,
            operation,
            status_id,
        }
    }
}
