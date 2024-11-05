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
pub struct EmployeeReportingTos {
    pub employee_reporting_to_id: Uuid,
pub employee_id: Uuid,
pub reporting_to_employee_id: Uuid,
pub start_date: DateTime<Utc>,
pub end_date: DateTime<Utc>,
pub status_id: Uuid

}

impl EmployeeReportingTos {
    pub const TABLE: &'static str = r#""Employee"."EmployeeReportingTos""#;
    pub const PK: &'static str = "EmployeeReportingToId";
    pub const COLUMNS_ARRAY: [&'static str; 6] = ["EmployeeReportingToId","EmployeeId","ReportingToEmployeeId","StartDate","EndDate","StatusId"];

    pub fn get_id(&self) -> Uuid {
        self.employee_reporting_to_id.clone()
    }

    pub fn get_args(&self) -> PgArguments {
        let mut args = PgArguments::default();
        let _ = args.add(self.employee_reporting_to_id.clone());
let _ = args.add(self.employee_id.clone());
let _ = args.add(self.reporting_to_employee_id.clone());
let _ = args.add(self.start_date.clone());
let _ = args.add(self.end_date.clone());
let _ = args.add(self.status_id.clone());

        args
    }

    pub fn new(employee_reporting_to_id: Uuid,employee_id: Uuid,reporting_to_employee_id: Uuid,start_date: DateTime<Utc>,end_date: DateTime<Utc>,status_id: Uuid) -> Self {
        Self {
            employee_reporting_to_id,
employee_id,
reporting_to_employee_id,
start_date,
end_date,
status_id

        }
    }
}

impl PartialEq for EmployeeReportingTos {
    fn eq(&self, other: &Self) -> bool {
        self.employee_reporting_to_id == other.employee_reporting_to_id
    }
}

impl Model for EmployeeReportingTos {
    fn from_row(row: &PgRow) -> EmployeeReportingTos
    where
        Self: Sized,
    {
        let employee_reporting_to_id = row.get("EmployeeReportingToId");
let employee_id = row.get("EmployeeId");
let reporting_to_employee_id = row.get("ReportingToEmployeeId");
let start_date = row.get("StartDate");
let end_date = row.get("EndDate");
let status_id = row.get("StatusId");


        Self {
            employee_reporting_to_id,
employee_id,
reporting_to_employee_id,
start_date,
end_date,
status_id

        }
    }
}
