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
pub struct Packages {
    pub package_id: Uuid,
pub abbreviation: String,
pub full_name: String,
pub start_date: DateTime<Utc>,
pub end_date: DateTime<Utc>,
pub term_and_conditions: String,
pub status_id: Uuid,
pub package_type_id: Uuid,
pub package_rule_id: Uuid

}

impl Packages {
    pub const TABLE: &'static str = r#""Customer"."Packages""#;
    pub const PK: &'static str = r#"PackageId::TEXT=$1"#;
    pub const COLUMNS: &'static str = r#""PackageId","Abbreviation","FullName","StartDate","EndDate","TermAndConditions","StatusId","PackageTypeId","PackageRuleId""#;
    pub const COLUMNS_UPDATE: &'static str = r#""PackageId"=$1,"Abbreviation"=$2,"FullName"=$3,"StartDate"=$4,"EndDate"=$5,"TermAndConditions"=$6,"StatusId"=$7,"PackageTypeId"=$8,"PackageRuleId"=$9 WHERE "PackageId"=$1"#;

    pub fn get_id(&self) -> Uuid {
        self.package_id.clone()
    }

    pub fn new(package_id: Uuid,abbreviation: String,full_name: String,start_date: DateTime<Utc>,end_date: DateTime<Utc>,term_and_conditions: String,status_id: Uuid,package_type_id: Uuid,package_rule_id: Uuid) -> Self {
        Self {
            package_id,
abbreviation,
full_name,
start_date,
end_date,
term_and_conditions,
status_id,
package_type_id,
package_rule_id

        }
    }
}

impl PartialEq for Packages {
    fn eq(&self, other: &Self) -> bool {
        self.package_id == other.package_id
    }
}

impl Model for Packages {
    fn from_row(row: &PgRow) -> Packages
    where
        Self: Sized,
    {
        let package_id = row.get("PackageId");
let abbreviation = row.get("Abbreviation");
let full_name = row.get("FullName");
let start_date = row.get("StartDate");
let end_date = row.get("EndDate");
let term_and_conditions = row.get("TermAndConditions");
let status_id = row.get("StatusId");
let package_type_id = row.get("PackageTypeId");
let package_rule_id = row.get("PackageRuleId");


        Self {
            package_id,
abbreviation,
full_name,
start_date,
end_date,
term_and_conditions,
status_id,
package_type_id,
package_rule_id

        }
    }
}
