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
    pub const PK: &'static str = "EmployeeExpenseId";
    pub const COLUMNS_ARRAY: [&'static str; 6] = ["EmployeeExpenseId","EmployeeId","ExpenseTypeId","ExpenseDate","ExpenseAmount","StatusId"];

    pub fn get_id(&self) -> Uuid {
        self.employee_expense_id.clone()
    }

    pub fn get_args(&self) -> PgArguments {
        let mut args = PgArguments::default();
        let _ = args.add(self.employee_expense_id.clone());
let _ = args.add(self.employee_id.clone());
let _ = args.add(self.expense_type_id.clone());
let _ = args.add(self.expense_date.clone());
let _ = args.add(self.expense_amount.clone());
let _ = args.add(self.status_id.clone());

        args
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
