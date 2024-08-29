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
pub struct GazettedHolidays {
    pub gazetted_holiday_id: Uuid,
pub title: String,
pub start_date: DateTime<Utc>,
pub end_date: DateTime<Utc>,
pub status_id: Uuid

}

impl GazettedHolidays {
    pub const TABLE: &'static str = r#""Attendance"."GazettedHolidays""#;
    pub const PK: &'static str = r#"GazettedHolidayId::TEXT=$1"#;
    pub const COLUMNS: &'static str = r#""GazettedHolidayId","Title","StartDate","EndDate","StatusId""#;
    pub const COLUMNS_UPDATE: &'static str = r#""GazettedHolidayId"=$1,"Title"=$2,"StartDate"=$3,"EndDate"=$4,"StatusId"=$5 WHERE "GazettedHolidayId"=$1"#;

    pub fn get_id(&self) -> Uuid {
        self.gazetted_holiday_id.clone()
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
