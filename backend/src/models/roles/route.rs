use serde::{Deserialize, Serialize};
pub use sqlx::{
    pool::PoolConnection,
    postgres::{PgArguments, PgPoolOptions, PgRow},
    Arguments, Error, PgPool, Postgres, Row,
};
use uuid::Uuid;
use crate::interfaces::irepository::Model;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Route {
    pub route_id: Uuid,
    pub route_name: String,
    pub display_name: String,
    pub operation: String,
    pub status_id: Uuid,
}

impl Route {
    pub const TABLE: &'static str = r#""Role"."Routes""#;
    pub const PK: &'static str = r#""RouteId"::TEXT=$1"#;
    pub const COLUMNS: &'static str =
        r#""RouteId", "RouteName", "DisplayName", "Operation", "StatusId""#;
    pub const COLUMNS_UPDATE: &'static str =
        r#""RouteName"=$2, "DisplayName"=$3, "Operation"=$4, "StatusId"=$5 WHERE "RouteId"=$1"#;

    pub fn get_id(&self) -> Uuid {
        self.route_id.clone()
    }

    pub fn new(
        route_id: Uuid,
        route_name: String,
        display_name: String,
        operation: String,
        status_id: Uuid,
    ) -> Self {
        Self {
            route_id,
            route_name,
            display_name,
            operation,
            status_id,
        }
    }
}

impl PartialEq for Route {
    fn eq(&self, other: &Self) -> bool {
        self.route_id == other.route_id
    }
}

impl Model for Route {
    fn from_row(row: &PgRow) -> Route
    where
        Self: Sized,
    {
        let route_id = row.get("RouteId");
        let route_name = row.get("RouteName");
        let display_name = row.get("DisplayName");
        let operation = row.get("Operation");
        let status_id = row.get("StatusId");

        Self {
            route_id,
            route_name,
            display_name,
            operation,
            status_id,
        }
    }
}
