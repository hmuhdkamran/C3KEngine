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
pub struct EmployeeDocumentApporovals {
    pub employee_document_apporoval_id: Uuid,
pub employee_id: Uuid,
pub document_type_id: Uuid,
pub status_id: Uuid

}

impl EmployeeDocumentApporovals {
    pub const TABLE: &'static str = r#""Employee"."EmployeeDocumentApporovals""#;
    pub const PK: &'static str = r#"EmployeeDocumentApporovalId::TEXT=$1"#;
    pub const COLUMNS: &'static str = r#""EmployeeDocumentApporovalId","EmployeeId","DocumentTypeId","StatusId""#;
    pub const COLUMNS_UPDATE: &'static str = r#""EmployeeDocumentApporovalId"=$1,"EmployeeId"=$2,"DocumentTypeId"=$3,"StatusId"=$4 WHERE "EmployeeDocumentApporovalId"=$1"#;

    pub fn get_id(&self) -> Uuid {
        self.employee_document_apporoval_id.clone()
    }

    pub fn new(employee_document_apporoval_id: Uuid,employee_id: Uuid,document_type_id: Uuid,status_id: Uuid) -> Self {
        Self {
            employee_document_apporoval_id,
employee_id,
document_type_id,
status_id

        }
    }
}

impl PartialEq for EmployeeDocumentApporovals {
    fn eq(&self, other: &Self) -> bool {
        self.employee_document_apporoval_id == other.employee_document_apporoval_id
    }
}

impl Model for EmployeeDocumentApporovals {
    fn from_row(row: &PgRow) -> EmployeeDocumentApporovals
    where
        Self: Sized,
    {
        let employee_document_apporoval_id = row.get("EmployeeDocumentApporovalId");
let employee_id = row.get("EmployeeId");
let document_type_id = row.get("DocumentTypeId");
let status_id = row.get("StatusId");


        Self {
            employee_document_apporoval_id,
employee_id,
document_type_id,
status_id

        }
    }
}
