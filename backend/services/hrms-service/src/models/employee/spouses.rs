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
    pub const PK: &'static str = "SpouseId";
    pub const COLUMNS_ARRAY: [&'static str; 8] = ["SpouseId","FullName","ContactNumber","ProfessionId","DesignationId","PersonalInformationId","StatusId","SpouseTypeId"];

    pub fn get_id(&self) -> Uuid {
        self.spouse_id.clone()
    }

    pub fn get_args(&self) -> PgArguments {
        let mut args = PgArguments::default();
        let _ = args.add(self.spouse_id.clone());
let _ = args.add(self.full_name.clone());
let _ = args.add(self.contact_number.clone());
let _ = args.add(self.profession_id.clone());
let _ = args.add(self.designation_id.clone());
let _ = args.add(self.personal_information_id.clone());
let _ = args.add(self.status_id.clone());
let _ = args.add(self.spouse_type_id.clone());

        args
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
