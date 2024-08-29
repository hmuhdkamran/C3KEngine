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
pub struct EmployeeExpenses {
    pub employee_expense_id: Uuid,
pub employee_id: Uuid,
pub expense_type_id: Uuid,
pub expense_date: DateTime<Utc>,
pub expense_amount: f64,
pub status_id: Uuid

}

impl EmployeeExpenses {
    pub const TABLE: &'static str = r#""Payroll"."EmployeeExpenses""#;
    pub const PK: &'static str = r#"EmployeeExpenseId::TEXT=$1"#;
    pub const COLUMNS: &'static str = r#""EmployeeExpenseId","EmployeeId","ExpenseTypeId","ExpenseDate","ExpenseAmount","StatusId""#;
    pub const COLUMNS_UPDATE: &'static str = r#""EmployeeExpenseId"=$1,"EmployeeId"=$2,"ExpenseTypeId"=$3,"ExpenseDate"=$4,"ExpenseAmount"=$5,"StatusId"=$6 WHERE "EmployeeExpenseId"=$1"#;

    pub fn get_id(&self) -> Uuid {
        self.employee_expense_id.clone()
    }

    pub fn new(employee_expense_id: Uuid,employee_id: Uuid,expense_type_id: Uuid,expense_date: DateTime<Utc>,expense_amount: f64,status_id: Uuid) -> Self {
        Self {
            employee_expense_id,
employee_id,
expense_type_id,
expense_date,
expense_amount,
status_id

        }
    }
}

impl PartialEq for EmployeeExpenses {
    fn eq(&self, other: &Self) -> bool {
        self.employee_expense_id == other.employee_expense_id
    }
}

impl Model for EmployeeExpenses {
    fn from_row(row: &PgRow) -> EmployeeExpenses
    where
        Self: Sized,
    {
        let employee_expense_id = row.get("EmployeeExpenseId");
let employee_id = row.get("EmployeeId");
let expense_type_id = row.get("ExpenseTypeId");
let expense_date = row.get("ExpenseDate");
let expense_amount = row.get("ExpenseAmount");
let status_id = row.get("StatusId");


        Self {
            employee_expense_id,
employee_id,
expense_type_id,
expense_date,
expense_amount,
status_id

        }
    }
}
