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
pub struct TravelRequests {
    pub travel_request_id: Uuid,
pub employee_id: Uuid,
pub start_date: DateTime<Utc>,
pub end_date: DateTime<Utc>,
pub purpose: String,
pub request_date: DateTime<Utc>,
pub travel_from: String,
pub travel_to: String

}

impl TravelRequests {
    pub const TABLE: &'static str = r#""Payroll"."TravelRequests""#;
    pub const PK: &'static str = r#"TravelRequestId::TEXT=$1"#;
    pub const COLUMNS: &'static str = r#""TravelRequestId","EmployeeId","StartDate","EndDate","Purpose","RequestDate","TravelFrom","TravelTo""#;
    pub const COLUMNS_UPDATE: &'static str = r#""TravelRequestId"=$1,"EmployeeId"=$2,"StartDate"=$3,"EndDate"=$4,"Purpose"=$5,"RequestDate"=$6,"TravelFrom"=$7,"TravelTo"=$8 WHERE "TravelRequestId"=$1"#;

    pub fn get_id(&self) -> Uuid {
        self.travel_request_id.clone()
    }

    pub fn new(travel_request_id: Uuid,employee_id: Uuid,start_date: DateTime<Utc>,end_date: DateTime<Utc>,purpose: String,request_date: DateTime<Utc>,travel_from: String,travel_to: String) -> Self {
        Self {
            travel_request_id,
employee_id,
start_date,
end_date,
purpose,
request_date,
travel_from,
travel_to

        }
    }
}

impl PartialEq for TravelRequests {
    fn eq(&self, other: &Self) -> bool {
        self.travel_request_id == other.travel_request_id
    }
}

impl Model for TravelRequests {
    fn from_row(row: &PgRow) -> TravelRequests
    where
        Self: Sized,
    {
        let travel_request_id = row.get("TravelRequestId");
let employee_id = row.get("EmployeeId");
let start_date = row.get("StartDate");
let end_date = row.get("EndDate");
let purpose = row.get("Purpose");
let request_date = row.get("RequestDate");
let travel_from = row.get("TravelFrom");
let travel_to = row.get("TravelTo");


        Self {
            travel_request_id,
employee_id,
start_date,
end_date,
purpose,
request_date,
travel_from,
travel_to

        }
    }
}
