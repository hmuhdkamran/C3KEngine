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
    pub const PK: &'static str = r#"OvertimeRequestApprovalId::TEXT=$1"#;
    pub const COLUMNS: &'static str = r#""OvertimeRequestApprovalId","OvertimeRequestId","ApprovedBy","ApprovedDate","ApplicationStatusId","Comments","StatusId""#;
    pub const COLUMNS_UPDATE: &'static str = r#""OvertimeRequestApprovalId"=$1,"OvertimeRequestId"=$2,"ApprovedBy"=$3,"ApprovedDate"=$4,"ApplicationStatusId"=$5,"Comments"=$6,"StatusId"=$7 WHERE "OvertimeRequestApprovalId"=$1"#;

    pub fn get_id(&self) -> Uuid {
        self.overtime_request_approval_id.clone()
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
