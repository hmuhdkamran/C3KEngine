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
pub struct GazettedHolidays {
    pub gazetted_holiday_id: Uuid,
pub title: String,
pub start_date: DateTime<Utc>,
pub end_date: DateTime<Utc>,
pub status_id: Uuid

}

impl GazettedHolidays {
    pub const TABLE: &'static str = r#""Attendance"."GazettedHolidays""#;
    pub const PK: &'static str = "GazettedHolidayId";
    pub const COLUMNS_ARRAY: [&'static str; 5] = ["GazettedHolidayId","Title","StartDate","EndDate","StatusId"];

    pub fn get_id(&self) -> Uuid {
        self.gazetted_holiday_id.clone()
    }

    pub fn get_args(&self) -> PgArguments {
        let mut args = PgArguments::default();
        let _ = args.add(self.gazetted_holiday_id.clone());
let _ = args.add(self.title.clone());
let _ = args.add(self.start_date.clone());
let _ = args.add(self.end_date.clone());
let _ = args.add(self.status_id.clone());

        args
    }

    pub fn new(gazetted_holiday_id: Uuid,title: String,start_date: DateTime<Utc>,end_date: DateTime<Utc>,status_id: Uuid) -> Self {
        Self {
            gazetted_holiday_id,
title,
start_date,
end_date,
status_id

        }
    }
}

impl PartialEq for GazettedHolidays {
    fn eq(&self, other: &Self) -> bool {
        self.gazetted_holiday_id == other.gazetted_holiday_id
    }
}

impl Model for GazettedHolidays {
    fn from_row(row: &PgRow) -> GazettedHolidays
    where
        Self: Sized,
    {
        let gazetted_holiday_id = row.get("GazettedHolidayId");
let title = row.get("Title");
let start_date = row.get("StartDate");
let end_date = row.get("EndDate");
let status_id = row.get("StatusId");


        Self {
            gazetted_holiday_id,
title,
start_date,
end_date,
status_id

        }
    }
}
