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
pub struct PackageRules {
    pub package_rule_id: Uuid,
pub abbreviation: String,
pub full_name: String,
pub percentage_flat: bool,
pub one_point_value: f64,
pub status_id: Uuid

}

impl PackageRules {
    pub const TABLE: &'static str = r#""Customer"."PackageRules""#;
    pub const PK: &'static str = "PackageRuleId";
    pub const COLUMNS_ARRAY: [&'static str; 6] = ["PackageRuleId","Abbreviation","FullName","PercentageFlat","OnePointValue","StatusId"];

    pub fn get_id(&self) -> Uuid {
        self.package_rule_id.clone()
    }

    pub fn get_args(&self) -> PgArguments {
        let mut args = PgArguments::default();
        let _ = args.add(self.package_rule_id.clone());
let _ = args.add(self.abbreviation.clone());
let _ = args.add(self.full_name.clone());
let _ = args.add(self.percentage_flat.clone());
let _ = args.add(self.one_point_value.clone());
let _ = args.add(self.status_id.clone());

        args
    }

    pub fn new(package_rule_id: Uuid,abbreviation: String,full_name: String,percentage_flat: bool,one_point_value: f64,status_id: Uuid) -> Self {
        Self {
            package_rule_id,
abbreviation,
full_name,
percentage_flat,
one_point_value,
status_id

        }
    }
}

impl PartialEq for PackageRules {
    fn eq(&self, other: &Self) -> bool {
        self.package_rule_id == other.package_rule_id
    }
}

impl Model for PackageRules {
    fn from_row(row: &PgRow) -> PackageRules
    where
        Self: Sized,
    {
        let package_rule_id = row.get("PackageRuleId");
let abbreviation = row.get("Abbreviation");
let full_name = row.get("FullName");
let percentage_flat = row.get("PercentageFlat");
let one_point_value = row.get("OnePointValue");
let status_id = row.get("StatusId");


        Self {
            package_rule_id,
abbreviation,
full_name,
percentage_flat,
one_point_value,
status_id

        }
    }
}
