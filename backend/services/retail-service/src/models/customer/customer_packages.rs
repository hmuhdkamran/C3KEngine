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
pub struct CustomerPackages {
    pub customer_package_id: Uuid,
pub customer_id: Uuid,
pub package_id: Uuid,
pub start_date: DateTime<Utc>,
pub end_date: DateTime<Utc>,
pub status_id: Uuid

}

impl CustomerPackages {
    pub const TABLE: &'static str = r#""Customer"."CustomerPackages""#;
    pub const PK: &'static str = r#"CustomerPackageId::TEXT=$1"#;
    pub const COLUMNS: &'static str = r#""CustomerPackageId","CustomerId","PackageId","StartDate","EndDate","StatusId""#;
    pub const COLUMNS_UPDATE: &'static str = r#""CustomerPackageId"=$1,"CustomerId"=$2,"PackageId"=$3,"StartDate"=$4,"EndDate"=$5,"StatusId"=$6 WHERE "CustomerPackageId"=$1"#;

    pub fn get_id(&self) -> Uuid {
        self.customer_package_id.clone()
    }

    pub fn new(customer_package_id: Uuid,customer_id: Uuid,package_id: Uuid,start_date: DateTime<Utc>,end_date: DateTime<Utc>,status_id: Uuid) -> Self {
        Self {
            customer_package_id,
customer_id,
package_id,
start_date,
end_date,
status_id

        }
    }
}

impl PartialEq for CustomerPackages {
    fn eq(&self, other: &Self) -> bool {
        self.customer_package_id == other.customer_package_id
    }
}

impl Model for CustomerPackages {
    fn from_row(row: &PgRow) -> CustomerPackages
    where
        Self: Sized,
    {
        let customer_package_id = row.get("CustomerPackageId");
let customer_id = row.get("CustomerId");
let package_id = row.get("PackageId");
let start_date = row.get("StartDate");
let end_date = row.get("EndDate");
let status_id = row.get("StatusId");


        Self {
            customer_package_id,
customer_id,
package_id,
start_date,
end_date,
status_id

        }
    }
}
