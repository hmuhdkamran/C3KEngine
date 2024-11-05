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
pub struct TravelRequestApprovals {
    pub travel_request_approval_id: Uuid,
pub travel_request_id: Uuid,
pub approved_by: Uuid,
pub approval_date: DateTime<Utc>,
pub application_status_id: Uuid,
pub comments: String,
pub status_id: Uuid

}

impl TravelRequestApprovals {
    pub const TABLE: &'static str = r#""Payroll"."TravelRequestApprovals""#;
    pub const PK: &'static str = "TravelRequestApprovalId";
    pub const COLUMNS_ARRAY: [&'static str; 7] = ["TravelRequestApprovalId","TravelRequestId","ApprovedBy","ApprovalDate","ApplicationStatusId","Comments","StatusId"];

    pub fn get_id(&self) -> Uuid {
        self.travel_request_approval_id.clone()
    }

    pub fn get_args(&self) -> PgArguments {
        let mut args = PgArguments::default();
        let _ = args.add(self.travel_request_approval_id.clone());
let _ = args.add(self.travel_request_id.clone());
let _ = args.add(self.approved_by.clone());
let _ = args.add(self.approval_date.clone());
let _ = args.add(self.application_status_id.clone());
let _ = args.add(self.comments.clone());
let _ = args.add(self.status_id.clone());

        args
    }

    pub fn new(travel_request_approval_id: Uuid,travel_request_id: Uuid,approved_by: Uuid,approval_date: DateTime<Utc>,application_status_id: Uuid,comments: String,status_id: Uuid) -> Self {
        Self {
            travel_request_approval_id,
travel_request_id,
approved_by,
approval_date,
application_status_id,
comments,
status_id

        }
    }
}

impl PartialEq for TravelRequestApprovals {
    fn eq(&self, other: &Self) -> bool {
        self.travel_request_approval_id == other.travel_request_approval_id
    }
}

impl Model for TravelRequestApprovals {
    fn from_row(row: &PgRow) -> TravelRequestApprovals
    where
        Self: Sized,
    {
        let travel_request_approval_id = row.get("TravelRequestApprovalId");
let travel_request_id = row.get("TravelRequestId");
let approved_by = row.get("ApprovedBy");
let approval_date = row.get("ApprovalDate");
let application_status_id = row.get("ApplicationStatusId");
let comments = row.get("Comments");
let status_id = row.get("StatusId");


        Self {
            travel_request_approval_id,
travel_request_id,
approved_by,
approval_date,
application_status_id,
comments,
status_id

        }
    }
}
