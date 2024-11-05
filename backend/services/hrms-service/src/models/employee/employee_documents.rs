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
pub struct EmployeeDocuments {
    pub employee_document_id: Uuid,
pub full_name: String,
pub document_url: String,
pub employee_id: Uuid,
pub status_id: Uuid

}

impl EmployeeDocuments {
    pub const TABLE: &'static str = r#""Employee"."EmployeeDocuments""#;
    pub const PK: &'static str = "EmployeeDocumentId";
    pub const COLUMNS_ARRAY: [&'static str; 5] = ["EmployeeDocumentId","FullName","DocumentUrl","EmployeeId","StatusId"];

    pub fn get_id(&self) -> Uuid {
        self.employee_document_id.clone()
    }

    pub fn get_args(&self) -> PgArguments {
        let mut args = PgArguments::default();
        let _ = args.add(self.employee_document_id.clone());
let _ = args.add(self.full_name.clone());
let _ = args.add(self.document_url.clone());
let _ = args.add(self.employee_id.clone());
let _ = args.add(self.status_id.clone());

        args
    }

    pub fn new(employee_document_id: Uuid,full_name: String,document_url: String,employee_id: Uuid,status_id: Uuid) -> Self {
        Self {
            employee_document_id,
full_name,
document_url,
employee_id,
status_id

        }
    }
}

impl PartialEq for EmployeeDocuments {
    fn eq(&self, other: &Self) -> bool {
        self.employee_document_id == other.employee_document_id
    }
}

impl Model for EmployeeDocuments {
    fn from_row(row: &PgRow) -> EmployeeDocuments
    where
        Self: Sized,
    {
        let employee_document_id = row.get("EmployeeDocumentId");
let full_name = row.get("FullName");
let document_url = row.get("DocumentUrl");
let employee_id = row.get("EmployeeId");
let status_id = row.get("StatusId");


        Self {
            employee_document_id,
full_name,
document_url,
employee_id,
status_id

        }
    }
}
