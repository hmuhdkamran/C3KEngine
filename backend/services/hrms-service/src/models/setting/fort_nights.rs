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
pub struct FortNights {
    pub fort_night_id: Uuid,
pub fort_night_no: i32,
pub status_id: Uuid

}

impl FortNights {
    pub const TABLE: &'static str = r#""Setting"."FortNights""#;
    pub const PK: &'static str = "FortNightId";
    pub const COLUMNS_ARRAY: [&'static str; 3] = ["FortNightId","FortNightNo","StatusId"];

    pub fn get_id(&self) -> Uuid {
        self.fort_night_id.clone()
    }

    pub fn get_args(&self) -> PgArguments {
        let mut args = PgArguments::default();
        let _ = args.add(self.fort_night_id.clone());
let _ = args.add(self.fort_night_no.clone());
let _ = args.add(self.status_id.clone());

        args
    }

    pub fn new(fort_night_id: Uuid,fort_night_no: i32,status_id: Uuid) -> Self {
        Self {
            fort_night_id,
fort_night_no,
status_id

        }
    }
}

impl PartialEq for FortNights {
    fn eq(&self, other: &Self) -> bool {
        self.fort_night_id == other.fort_night_id
    }
}

impl Model for FortNights {
    fn from_row(row: &PgRow) -> FortNights
    where
        Self: Sized,
    {
        let fort_night_id = row.get("FortNightId");
let fort_night_no = row.get("FortNightNo");
let status_id = row.get("StatusId");


        Self {
            fort_night_id,
fort_night_no,
status_id

        }
    }
}
