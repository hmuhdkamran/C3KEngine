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
    pub const PK: &'static str = r#"DepartmentId::TEXT=$1"#;
    pub const COLUMNS: &'static str = r#""DepartmentId","FullName","GroupId","StatusId","Abbreviation","ParentDepartmentId","BusinessId""#;
    pub const COLUMNS_UPDATE: &'static str = r#""DepartmentId"=$1,"FullName"=$3,"GroupId"=$4,"StatusId"=$5,"Abbreviation"=$6,"ParentDepartmentId"=$7,"BusinessId"=$8 WHERE "DepartmentId"=$1"#;

    pub fn get_id(&self) -> Uuid {
        self.department_id.clone()
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
