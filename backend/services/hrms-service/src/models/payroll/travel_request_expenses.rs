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
pub struct TravelRequestExpenses {
    pub travel_request_expense_id: Uuid,
pub travel_request_id: Uuid,
pub expense_type_id: Uuid,
pub expense_amount: f64,
pub expensedate: DateTime<Utc>,
pub description: String,
pub status_id: Uuid

}

impl TravelRequestExpenses {
    pub const TABLE: &'static str = r#""Payroll"."TravelRequestExpenses""#;
    pub const PK: &'static str = "TravelRequestExpenseId";
    pub const COLUMNS_ARRAY: [&'static str; 7] = ["TravelRequestExpenseId","TravelRequestId","ExpenseTypeId","ExpenseAmount","Expensedate","Description","StatusId"];

    pub fn get_id(&self) -> Uuid {
        self.travel_request_expense_id.clone()
    }

    pub fn get_args(&self) -> PgArguments {
        let mut args = PgArguments::default();
        let _ = args.add(self.travel_request_expense_id.clone());
let _ = args.add(self.travel_request_id.clone());
let _ = args.add(self.expense_type_id.clone());
let _ = args.add(self.expense_amount.clone());
let _ = args.add(self.expensedate.clone());
let _ = args.add(self.description.clone());
let _ = args.add(self.status_id.clone());

        args
    }

    pub fn new(travel_request_expense_id: Uuid,travel_request_id: Uuid,expense_type_id: Uuid,expense_amount: f64,expensedate: DateTime<Utc>,description: String,status_id: Uuid) -> Self {
        Self {
            travel_request_expense_id,
travel_request_id,
expense_type_id,
expense_amount,
expensedate,
description,
status_id

        }
    }
}

impl PartialEq for TravelRequestExpenses {
    fn eq(&self, other: &Self) -> bool {
        self.travel_request_expense_id == other.travel_request_expense_id
    }
}

impl Model for TravelRequestExpenses {
    fn from_row(row: &PgRow) -> TravelRequestExpenses
    where
        Self: Sized,
    {
        let travel_request_expense_id = row.get("TravelRequestExpenseId");
let travel_request_id = row.get("TravelRequestId");
let expense_type_id = row.get("ExpenseTypeId");
let expense_amount = row.get("ExpenseAmount");
let expensedate = row.get("Expensedate");
let description = row.get("Description");
let status_id = row.get("StatusId");


        Self {
            travel_request_expense_id,
travel_request_id,
expense_type_id,
expense_amount,
expensedate,
description,
status_id

        }
    }
}
