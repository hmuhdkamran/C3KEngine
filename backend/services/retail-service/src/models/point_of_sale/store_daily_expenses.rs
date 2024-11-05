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
pub struct StoreDailyExpenses {
    pub store_daily_expense_id: Uuid,
pub expense_type_id: Uuid,
pub amount: f64,
pub description: String,
pub picture: String,
pub expense_date: DateTime<Utc>,
pub status_id: Uuid,
pub branch_id: Uuid

}

impl StoreDailyExpenses {
    pub const TABLE: &'static str = r#""PointOfSale"."StoreDailyExpenses""#;
    pub const PK: &'static str = "StoreDailyExpenseId";
    pub const COLUMNS_ARRAY: [&'static str; 8] = ["StoreDailyExpenseId","ExpenseTypeId","Amount","Description","Picture","ExpenseDate","StatusId","BranchId"];

    pub fn get_id(&self) -> Uuid {
        self.store_daily_expense_id.clone()
    }

    pub fn get_args(&self) -> PgArguments {
        let mut args = PgArguments::default();
        let _ = args.add(self.store_daily_expense_id.clone());
let _ = args.add(self.expense_type_id.clone());
let _ = args.add(self.amount.clone());
let _ = args.add(self.description.clone());
let _ = args.add(self.picture.clone());
let _ = args.add(self.expense_date.clone());
let _ = args.add(self.status_id.clone());
let _ = args.add(self.branch_id.clone());

        args
    }

    pub fn new(store_daily_expense_id: Uuid,expense_type_id: Uuid,amount: f64,description: String,picture: String,expense_date: DateTime<Utc>,status_id: Uuid,branch_id: Uuid) -> Self {
        Self {
            store_daily_expense_id,
expense_type_id,
amount,
description,
picture,
expense_date,
status_id,
branch_id

        }
    }
}

impl PartialEq for StoreDailyExpenses {
    fn eq(&self, other: &Self) -> bool {
        self.store_daily_expense_id == other.store_daily_expense_id
    }
}

impl Model for StoreDailyExpenses {
    fn from_row(row: &PgRow) -> StoreDailyExpenses
    where
        Self: Sized,
    {
        let store_daily_expense_id = row.get("StoreDailyExpenseId");
let expense_type_id = row.get("ExpenseTypeId");
let amount = row.get("Amount");
let description = row.get("Description");
let picture = row.get("Picture");
let expense_date = row.get("ExpenseDate");
let status_id = row.get("StatusId");
let branch_id = row.get("BranchId");


        Self {
            store_daily_expense_id,
expense_type_id,
amount,
description,
picture,
expense_date,
status_id,
branch_id

        }
    }
}
