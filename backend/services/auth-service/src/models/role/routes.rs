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
pub struct Routes {
    pub route_id: Uuid,
pub route_name: String,
pub display_name: String,
pub operation: String,
pub status_id: Uuid

}

impl Routes {
    pub const TABLE: &'static str = r#""Role"."Routes""#;
    pub const PK: &'static str = "RouteId";
    pub const COLUMNS_ARRAY: [&'static str; 5] = ["RouteId","RouteName","DisplayName","Operation","StatusId"];

    pub fn get_id(&self) -> Uuid {
        self.route_id.clone()
    }

    pub fn get_args(&self) -> PgArguments {
        let mut args = PgArguments::default();
        let _ = args.add(self.route_id.clone());
let _ = args.add(self.route_name.clone());
let _ = args.add(self.display_name.clone());
let _ = args.add(self.operation.clone());
let _ = args.add(self.status_id.clone());

        args
    }

    pub fn new(route_id: Uuid,route_name: String,display_name: String,operation: String,status_id: Uuid) -> Self {
        Self {
            route_id,
route_name,
display_name,
operation,
status_id

        }
    }
}

impl PartialEq for Routes {
    fn eq(&self, other: &Self) -> bool {
        self.route_id == other.route_id
    }
}

impl Model for Routes {
    fn from_row(row: &PgRow) -> Routes
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
status_id

        }
    }
}
