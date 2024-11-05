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
pub struct OvertimeRequestApprovals {
    pub overtime_request_approval_id: Uuid,
pub overtime_request_id: Uuid,
pub approved_by: Uuid,
pub approved_date: DateTime<Utc>,
pub application_status_id: Uuid,
pub comments: String,
pub status_id: Uuid

}

impl OvertimeRequestApprovals {
    pub const TABLE: &'static str = r#""Attendance"."OvertimeRequestApprovals""#;
    pub const PK: &'static str = "OvertimeRequestApprovalId";
    pub const COLUMNS_ARRAY: [&'static str; 7] = ["OvertimeRequestApprovalId","OvertimeRequestId","ApprovedBy","ApprovedDate","ApplicationStatusId","Comments","StatusId"];

    pub fn get_id(&self) -> Uuid {
        self.overtime_request_approval_id.clone()
    }

    pub fn get_args(&self) -> PgArguments {
        let mut args = PgArguments::default();
        let _ = args.add(self.overtime_request_approval_id.clone());
let _ = args.add(self.overtime_request_id.clone());
let _ = args.add(self.approved_by.clone());
let _ = args.add(self.approved_date.clone());
let _ = args.add(self.application_status_id.clone());
let _ = args.add(self.comments.clone());
let _ = args.add(self.status_id.clone());

        args
    }

    pub fn new(overtime_request_approval_id: Uuid,overtime_request_id: Uuid,approved_by: Uuid,approved_date: DateTime<Utc>,application_status_id: Uuid,comments: String,status_id: Uuid) -> Self {
        Self {
            overtime_request_approval_id,
overtime_request_id,
approved_by,
approved_date,
application_status_id,
comments,
status_id

        }
    }
}

impl PartialEq for OvertimeRequestApprovals {
    fn eq(&self, other: &Self) -> bool {
        self.overtime_request_approval_id == other.overtime_request_approval_id
    }
}

impl Model for OvertimeRequestApprovals {
    fn from_row(row: &PgRow) -> OvertimeRequestApprovals
    where
        Self: Sized,
    {
        let overtime_request_approval_id = row.get("OvertimeRequestApprovalId");
let overtime_request_id = row.get("OvertimeRequestId");
let approved_by = row.get("ApprovedBy");
let approved_date = row.get("ApprovedDate");
let application_status_id = row.get("ApplicationStatusId");
let comments = row.get("Comments");
let status_id = row.get("StatusId");


        Self {
            overtime_request_approval_id,
overtime_request_id,
approved_by,
approved_date,
application_status_id,
comments,
status_id

        }
    }
}
