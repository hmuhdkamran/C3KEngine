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
pub struct EmployeeIdTypes {
    pub employee_id_type_id: Uuid,
pub personal_information_id: Uuid,
pub id_type_id: Uuid,
pub description: String,
pub document_url: String,
pub status_id: Uuid

}

impl EmployeeIdTypes {
    pub const TABLE: &'static str = r#""Employee"."EmployeeIdTypes""#;
    pub const PK: &'static str = r#"EmployeeIdTypeId::TEXT=$1"#;
    pub const COLUMNS: &'static str = r#""EmployeeIdTypeId","PersonalInformationId","IdTypeId","Description","DocumentUrl","StatusId""#;
    pub const COLUMNS_UPDATE: &'static str = r#""EmployeeIdTypeId"=$1,"PersonalInformationId"=$2,"IdTypeId"=$3,"Description"=$4,"DocumentUrl"=$5,"StatusId"=$6 WHERE "EmployeeIdTypeId"=$1"#;

    pub fn get_id(&self) -> Uuid {
        self.employee_id_type_id.clone()
    }

    pub fn new(employee_id_type_id: Uuid,personal_information_id: Uuid,id_type_id: Uuid,description: String,document_url: String,status_id: Uuid) -> Self {
        Self {
            employee_id_type_id,
personal_information_id,
id_type_id,
description,
document_url,
status_id

        }
    }
}

impl PartialEq for EmployeeIdTypes {
    fn eq(&self, other: &Self) -> bool {
        self.employee_id_type_id == other.employee_id_type_id
    }
}

impl Model for EmployeeIdTypes {
    fn from_row(row: &PgRow) -> EmployeeIdTypes
    where
        Self: Sized,
    {
        let employee_id_type_id = row.get("EmployeeIdTypeId");
let personal_information_id = row.get("PersonalInformationId");
let id_type_id = row.get("IdTypeId");
let description = row.get("Description");
let document_url = row.get("DocumentUrl");
let status_id = row.get("StatusId");


        Self {
            employee_id_type_id,
personal_information_id,
id_type_id,
description,
document_url,
status_id

        }
    }
}
