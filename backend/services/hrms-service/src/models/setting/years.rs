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
pub struct Years {
    pub year_id: Uuid,
pub year_no: i32,
pub status: Uuid

}

impl Years {
    pub const TABLE: &'static str = r#""Setting"."Years""#;
    pub const PK: &'static str = r#"YearId::TEXT=$1"#;
    pub const COLUMNS: &'static str = r#""YearId","YearNo","Status""#;
    pub const COLUMNS_UPDATE: &'static str = r#""YearId"=$1,"YearNo"=$2,"Status"=$3 WHERE "YearId"=$1"#;

    pub fn get_id(&self) -> Uuid {
        self.year_id.clone()
    }

    pub fn new(year_id: Uuid,year_no: i32,status: Uuid) -> Self {
        Self {
            year_id,
year_no,
status

        }
    }
}

impl PartialEq for Years {
    fn eq(&self, other: &Self) -> bool {
        self.year_id == other.year_id
    }
}

impl Model for Years {
    fn from_row(row: &PgRow) -> Years
    where
        Self: Sized,
    {
        let year_id = row.get("YearId");
let year_no = row.get("YearNo");
let status = row.get("Status");


        Self {
            year_id,
year_no,
status

        }
    }
}
