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
pub struct ExpenseTypes {
    pub expense_type_id: Uuid,
pub abbreviation: String,
pub full_name: String,
pub status_id: Uuid

}

impl ExpenseTypes {
    pub const TABLE: &'static str = r#""Setup"."ExpenseTypes""#;
    pub const PK: &'static str = "ExpenseTypeId";
    pub const COLUMNS_ARRAY: [&'static str; 4] = ["ExpenseTypeId","Abbreviation","FullName","StatusId"];

    pub fn get_id(&self) -> Uuid {
        self.expense_type_id.clone()
    }

    pub fn get_args(&self) -> PgArguments {
        let mut args = PgArguments::default();
        let _ = args.add(self.expense_type_id.clone());
let _ = args.add(self.abbreviation.clone());
let _ = args.add(self.full_name.clone());
let _ = args.add(self.status_id.clone());

        args
    }

    pub fn new(expense_type_id: Uuid,abbreviation: String,full_name: String,status_id: Uuid) -> Self {
        Self {
            expense_type_id,
abbreviation,
full_name,
status_id

        }
    }
}

impl PartialEq for ExpenseTypes {
    fn eq(&self, other: &Self) -> bool {
        self.expense_type_id == other.expense_type_id
    }
}

impl Model for ExpenseTypes {
    fn from_row(row: &PgRow) -> ExpenseTypes
    where
        Self: Sized,
    {
        let expense_type_id = row.get("ExpenseTypeId");
let abbreviation = row.get("Abbreviation");
let full_name = row.get("FullName");
let status_id = row.get("StatusId");


        Self {
            expense_type_id,
abbreviation,
full_name,
status_id

        }
    }
}
