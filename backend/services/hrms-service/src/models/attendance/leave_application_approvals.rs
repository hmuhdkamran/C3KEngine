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
pub struct LeaveApplicationApprovals {
    pub leave_application_approval_id: Uuid,
pub leave_application_id: Uuid,
pub approved_by: Uuid,
pub approval_date: DateTime<Utc>,
pub application_status_id: Uuid,
pub status_id: Uuid,
pub comments: String

}

impl LeaveApplicationApprovals {
    pub const TABLE: &'static str = r#""Attendance"."LeaveApplicationApprovals""#;
    pub const PK: &'static str = r#"LeaveApplicationApprovalId::TEXT=$1"#;
    pub const COLUMNS: &'static str = r#""LeaveApplicationApprovalId","LeaveApplicationId","ApprovedBy","ApprovalDate","ApplicationStatusId","StatusId","Comments""#;
    pub const COLUMNS_UPDATE: &'static str = r#""LeaveApplicationApprovalId"=$1,"LeaveApplicationId"=$2,"ApprovedBy"=$3,"ApprovalDate"=$4,"ApplicationStatusId"=$5,"StatusId"=$6,"Comments"=$7 WHERE "LeaveApplicationApprovalId"=$1"#;

    pub fn get_id(&self) -> Uuid {
        self.leave_application_approval_id.clone()
    }

    pub fn new(leave_application_approval_id: Uuid,leave_application_id: Uuid,approved_by: Uuid,approval_date: DateTime<Utc>,application_status_id: Uuid,status_id: Uuid,comments: String) -> Self {
        Self {
            leave_application_approval_id,
leave_application_id,
approved_by,
approval_date,
application_status_id,
status_id,
comments

        }
    }
}

impl PartialEq for LeaveApplicationApprovals {
    fn eq(&self, other: &Self) -> bool {
        self.leave_application_approval_id == other.leave_application_approval_id
    }
}

impl Model for LeaveApplicationApprovals {
    fn from_row(row: &PgRow) -> LeaveApplicationApprovals
    where
        Self: Sized,
    {
        let leave_application_approval_id = row.get("LeaveApplicationApprovalId");
let leave_application_id = row.get("LeaveApplicationId");
let approved_by = row.get("ApprovedBy");
let approval_date = row.get("ApprovalDate");
let application_status_id = row.get("ApplicationStatusId");
let status_id = row.get("StatusId");
let comments = row.get("Comments");


        Self {
            leave_application_approval_id,
leave_application_id,
approved_by,
approval_date,
application_status_id,
status_id,
comments

        }
    }
}
