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
pub struct Weeks {
    pub week_id: Uuid,
pub week_no: i32,
pub status_id: Uuid

}

impl Weeks {
    pub const TABLE: &'static str = r#""Setting"."Weeks""#;
    pub const PK: &'static str = "WeekId";
    pub const COLUMNS_ARRAY: [&'static str; 3] = ["WeekId","WeekNo","StatusId"];

    pub fn get_id(&self) -> Uuid {
        self.week_id.clone()
    }

    pub fn get_args(&self) -> PgArguments {
        let mut args = PgArguments::default();
        let _ = args.add(self.week_id.clone());
let _ = args.add(self.week_no.clone());
let _ = args.add(self.status_id.clone());

        args
    }

    pub fn new(week_id: Uuid,week_no: i32,status_id: Uuid) -> Self {
        Self {
            week_id,
week_no,
status_id

        }
    }
}

impl PartialEq for Weeks {
    fn eq(&self, other: &Self) -> bool {
        self.week_id == other.week_id
    }
}

impl Model for Weeks {
    fn from_row(row: &PgRow) -> Weeks
    where
        Self: Sized,
    {
        let week_id = row.get("WeekId");
let week_no = row.get("WeekNo");
let status_id = row.get("StatusId");


        Self {
            week_id,
week_no,
status_id

        }
    }
}
