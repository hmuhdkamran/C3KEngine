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
pub struct ExpenseApprovals {
    pub expense_approval_id: Uuid,
pub employee_expense_id: Uuid,
pub approved_by: Uuid,
pub approval_date: DateTime<Utc>,
pub application_status_id: Uuid,
pub status_id: Uuid

}

impl ExpenseApprovals {
    pub const TABLE: &'static str = r#""Payroll"."ExpenseApprovals""#;
    pub const PK: &'static str = "ExpenseApprovalId";
    pub const COLUMNS_ARRAY: [&'static str; 6] = ["ExpenseApprovalId","EmployeeExpenseId","ApprovedBy","ApprovalDate","ApplicationStatusId","StatusId"];

    pub fn get_id(&self) -> Uuid {
        self.expense_approval_id.clone()
    }

    pub fn get_args(&self) -> PgArguments {
        let mut args = PgArguments::default();
        let _ = args.add(self.expense_approval_id.clone());
let _ = args.add(self.employee_expense_id.clone());
let _ = args.add(self.approved_by.clone());
let _ = args.add(self.approval_date.clone());
let _ = args.add(self.application_status_id.clone());
let _ = args.add(self.status_id.clone());

        args
    }

    pub fn new(expense_approval_id: Uuid,employee_expense_id: Uuid,approved_by: Uuid,approval_date: DateTime<Utc>,application_status_id: Uuid,status_id: Uuid) -> Self {
        Self {
            expense_approval_id,
employee_expense_id,
approved_by,
approval_date,
application_status_id,
status_id

        }
    }
}

impl PartialEq for ExpenseApprovals {
    fn eq(&self, other: &Self) -> bool {
        self.expense_approval_id == other.expense_approval_id
    }
}

impl Model for ExpenseApprovals {
    fn from_row(row: &PgRow) -> ExpenseApprovals
    where
        Self: Sized,
    {
        let expense_approval_id = row.get("ExpenseApprovalId");
let employee_expense_id = row.get("EmployeeExpenseId");
let approved_by = row.get("ApprovedBy");
let approval_date = row.get("ApprovalDate");
let application_status_id = row.get("ApplicationStatusId");
let status_id = row.get("StatusId");


        Self {
            expense_approval_id,
employee_expense_id,
approved_by,
approval_date,
application_status_id,
status_id

        }
    }
}
