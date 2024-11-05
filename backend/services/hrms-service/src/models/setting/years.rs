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
pub struct Years {
    pub year_id: Uuid,
pub year_no: i32,
pub status: Uuid

}

impl Years {
    pub const TABLE: &'static str = r#""Setting"."Years""#;
    pub const PK: &'static str = "YearId";
    pub const COLUMNS_ARRAY: [&'static str; 3] = ["YearId","YearNo","Status"];

    pub fn get_id(&self) -> Uuid {
        self.year_id.clone()
    }

    pub fn get_args(&self) -> PgArguments {
        let mut args = PgArguments::default();
        let _ = args.add(self.year_id.clone());
let _ = args.add(self.year_no.clone());
let _ = args.add(self.status.clone());

        args
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
