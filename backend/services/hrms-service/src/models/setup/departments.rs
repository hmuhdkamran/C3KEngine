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
pub struct Departments {
    pub department_id: Uuid,
pub full_name: String,
pub group_id: Uuid,
pub status_id: Uuid,
pub abbreviation: String,
pub parent_department_id: Uuid,
pub business_id: Uuid

}

impl Departments {
    pub const TABLE: &'static str = r#""Setup"."Departments""#;
    pub const PK: &'static str = "DepartmentId";
    pub const COLUMNS_ARRAY: [&'static str; 7] = ["DepartmentId","FullName","GroupId","StatusId","Abbreviation","ParentDepartmentId","BusinessId"];

    pub fn get_id(&self) -> Uuid {
        self.department_id.clone()
    }

    pub fn get_args(&self) -> PgArguments {
        let mut args = PgArguments::default();
        let _ = args.add(self.department_id.clone());
let _ = args.add(self.full_name.clone());
let _ = args.add(self.group_id.clone());
let _ = args.add(self.status_id.clone());
let _ = args.add(self.abbreviation.clone());
let _ = args.add(self.parent_department_id.clone());
let _ = args.add(self.business_id.clone());

        args
    }

    pub fn new(department_id: Uuid,full_name: String,group_id: Uuid,status_id: Uuid,abbreviation: String,parent_department_id: Uuid,business_id: Uuid) -> Self {
        Self {
            department_id,
full_name,
group_id,
status_id,
abbreviation,
parent_department_id,
business_id

        }
    }
}

impl PartialEq for Departments {
    fn eq(&self, other: &Self) -> bool {
        self.department_id == other.department_id
    }
}

impl Model for Departments {
    fn from_row(row: &PgRow) -> Departments
    where
        Self: Sized,
    {
        let department_id = row.get("DepartmentId");
let full_name = row.get("FullName");
let group_id = row.get("GroupId");
let status_id = row.get("StatusId");
let abbreviation = row.get("Abbreviation");
let parent_department_id = row.get("ParentDepartmentId");
let business_id = row.get("BusinessId");


        Self {
            department_id,
full_name,
group_id,
status_id,
abbreviation,
parent_department_id,
business_id

        }
    }
}
