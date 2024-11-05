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
pub struct ShortLeaveApporvals {
    pub short_leave_apporval_id: Uuid,
pub short_leave_id: Uuid,
pub approved_by: Uuid,
pub approval_date: DateTime<Utc>,
pub application_status_id: Uuid,
pub comments: String,
pub status_id: Uuid

}

impl ShortLeaveApporvals {
    pub const TABLE: &'static str = r#""Attendance"."ShortLeaveApporvals""#;
    pub const PK: &'static str = "ShortLeaveApporvalId";
    pub const COLUMNS_ARRAY: [&'static str; 7] = ["ShortLeaveApporvalId","ShortLeaveId","ApprovedBy","ApprovalDate","ApplicationStatusId","Comments","StatusId"];

    pub fn get_id(&self) -> Uuid {
        self.short_leave_apporval_id.clone()
    }

    pub fn get_args(&self) -> PgArguments {
        let mut args = PgArguments::default();
        let _ = args.add(self.short_leave_apporval_id.clone());
let _ = args.add(self.short_leave_id.clone());
let _ = args.add(self.approved_by.clone());
let _ = args.add(self.approval_date.clone());
let _ = args.add(self.application_status_id.clone());
let _ = args.add(self.comments.clone());
let _ = args.add(self.status_id.clone());

        args
    }

    pub fn new(short_leave_apporval_id: Uuid,short_leave_id: Uuid,approved_by: Uuid,approval_date: DateTime<Utc>,application_status_id: Uuid,comments: String,status_id: Uuid) -> Self {
        Self {
            short_leave_apporval_id,
short_leave_id,
approved_by,
approval_date,
application_status_id,
comments,
status_id

        }
    }
}

impl PartialEq for ShortLeaveApporvals {
    fn eq(&self, other: &Self) -> bool {
        self.short_leave_apporval_id == other.short_leave_apporval_id
    }
}

impl Model for ShortLeaveApporvals {
    fn from_row(row: &PgRow) -> ShortLeaveApporvals
    where
        Self: Sized,
    {
        let short_leave_apporval_id = row.get("ShortLeaveApporvalId");
let short_leave_id = row.get("ShortLeaveId");
let approved_by = row.get("ApprovedBy");
let approval_date = row.get("ApprovalDate");
let application_status_id = row.get("ApplicationStatusId");
let comments = row.get("Comments");
let status_id = row.get("StatusId");


        Self {
            short_leave_apporval_id,
short_leave_id,
approved_by,
approval_date,
application_status_id,
comments,
status_id

        }
    }
}
