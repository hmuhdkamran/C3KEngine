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
    pub const PK: &'static str = "EmployeeIdTypeId";
    pub const COLUMNS_ARRAY: [&'static str; 6] = ["EmployeeIdTypeId","PersonalInformationId","IdTypeId","Description","DocumentUrl","StatusId"];

    pub fn get_id(&self) -> Uuid {
        self.employee_id_type_id.clone()
    }

    pub fn get_args(&self) -> PgArguments {
        let mut args = PgArguments::default();
        let _ = args.add(self.employee_id_type_id.clone());
let _ = args.add(self.personal_information_id.clone());
let _ = args.add(self.id_type_id.clone());
let _ = args.add(self.description.clone());
let _ = args.add(self.document_url.clone());
let _ = args.add(self.status_id.clone());

        args
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
