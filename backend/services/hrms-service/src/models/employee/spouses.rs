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
pub struct Spouses {
    pub spouse_id: Uuid,
pub full_name: String,
pub contact_number: String,
pub profession_id: Uuid,
pub designation_id: Uuid,
pub personal_information_id: Uuid,
pub status_id: Uuid,
pub spouse_type_id: Uuid

}

impl Spouses {
    pub const TABLE: &'static str = r#""Employee"."Spouses""#;
    pub const PK: &'static str = r#"SpouseId::TEXT=$1"#;
    pub const COLUMNS: &'static str = r#""SpouseId","FullName","ContactNumber","ProfessionId","DesignationId","PersonalInformationId","StatusId","SpouseTypeId""#;
    pub const COLUMNS_UPDATE: &'static str = r#""SpouseId"=$1,"FullName"=$2,"ContactNumber"=$3,"ProfessionId"=$4,"DesignationId"=$5,"PersonalInformationId"=$6,"StatusId"=$8,"SpouseTypeId"=$9 WHERE "SpouseId"=$1"#;

    pub fn get_id(&self) -> Uuid {
        self.spouse_id.clone()
    }

    pub fn new(spouse_id: Uuid,full_name: String,contact_number: String,profession_id: Uuid,designation_id: Uuid,personal_information_id: Uuid,status_id: Uuid,spouse_type_id: Uuid) -> Self {
        Self {
            spouse_id,
full_name,
contact_number,
profession_id,
designation_id,
personal_information_id,
status_id,
spouse_type_id

        }
    }
}

impl PartialEq for Spouses {
    fn eq(&self, other: &Self) -> bool {
        self.spouse_id == other.spouse_id
    }
}

impl Model for Spouses {
    fn from_row(row: &PgRow) -> Spouses
    where
        Self: Sized,
    {
        let spouse_id = row.get("SpouseId");
let full_name = row.get("FullName");
let contact_number = row.get("ContactNumber");
let profession_id = row.get("ProfessionId");
let designation_id = row.get("DesignationId");
let personal_information_id = row.get("PersonalInformationId");
let status_id = row.get("StatusId");
let spouse_type_id = row.get("SpouseTypeId");


        Self {
            spouse_id,
full_name,
contact_number,
profession_id,
designation_id,
personal_information_id,
status_id,
spouse_type_id

        }
    }
}
