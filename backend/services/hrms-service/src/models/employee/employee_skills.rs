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
pub struct EmployeeSkills {
    pub employee_skill_id: Uuid,
pub personal_information_id: Uuid,
pub skill_id: Uuid,
pub status_id: Uuid

}

impl EmployeeSkills {
    pub const TABLE: &'static str = r#""Employee"."EmployeeSkills""#;
    pub const PK: &'static str = r#"EmployeeSkillId::TEXT=$1"#;
    pub const COLUMNS: &'static str = r#""EmployeeSkillId","PersonalInformationId","SkillId","StatusId""#;
    pub const COLUMNS_UPDATE: &'static str = r#""EmployeeSkillId"=$1,"PersonalInformationId"=$2,"SkillId"=$3,"StatusId"=$4 WHERE "EmployeeSkillId"=$1"#;

    pub fn get_id(&self) -> Uuid {
        self.employee_skill_id.clone()
    }

    pub fn new(employee_skill_id: Uuid,personal_information_id: Uuid,skill_id: Uuid,status_id: Uuid) -> Self {
        Self {
            employee_skill_id,
personal_information_id,
skill_id,
status_id

        }
    }
}

impl PartialEq for EmployeeSkills {
    fn eq(&self, other: &Self) -> bool {
        self.employee_skill_id == other.employee_skill_id
    }
}

impl Model for EmployeeSkills {
    fn from_row(row: &PgRow) -> EmployeeSkills
    where
        Self: Sized,
    {
        let employee_skill_id = row.get("EmployeeSkillId");
let personal_information_id = row.get("PersonalInformationId");
let skill_id = row.get("SkillId");
let status_id = row.get("StatusId");


        Self {
            employee_skill_id,
personal_information_id,
skill_id,
status_id

        }
    }
}
