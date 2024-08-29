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
    pub const PK: &'static str = r#"ExpenseApprovalId::TEXT=$1"#;
    pub const COLUMNS: &'static str = r#""ExpenseApprovalId","EmployeeExpenseId","ApprovedBy","ApprovalDate","ApplicationStatusId","StatusId""#;
    pub const COLUMNS_UPDATE: &'static str = r#""ExpenseApprovalId"=$1,"EmployeeExpenseId"=$2,"ApprovedBy"=$3,"ApprovalDate"=$4,"ApplicationStatusId"=$5,"StatusId"=$6 WHERE "ExpenseApprovalId"=$1"#;

    pub fn get_id(&self) -> Uuid {
        self.expense_approval_id.clone()
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
