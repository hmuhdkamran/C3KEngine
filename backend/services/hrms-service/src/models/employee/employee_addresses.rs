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
pub struct EmployeeAddresses {
    pub employee_address_id: Uuid,
pub employee_id: Uuid,
pub street_address: String,
pub city_id: Uuid,
pub state_id: Uuid,
pub country_id: Uuid,
pub address_type_id: Uuid,
pub is_primary: bool,
pub status_id: Uuid

}

impl EmployeeAddresses {
    pub const TABLE: &'static str = r#""Employee"."EmployeeAddresses""#;
    pub const PK: &'static str = r#"EmployeeAddressId::TEXT=$1"#;
    pub const COLUMNS: &'static str = r#""EmployeeAddressId","EmployeeId","StreetAddress ","CityId","StateId","CountryId","AddressTypeId","IsPrimary","StatusId""#;
    pub const COLUMNS_UPDATE: &'static str = r#""EmployeeAddressId"=$1,"EmployeeId"=$2,"StreetAddress "=$3,"CityId"=$4,"StateId"=$5,"CountryId"=$6,"AddressTypeId"=$7,"IsPrimary"=$8,"StatusId"=$9 WHERE "EmployeeAddressId"=$1"#;

    pub fn get_id(&self) -> Uuid {
        self.employee_address_id.clone()
    }

    pub fn new(employee_address_id: Uuid,employee_id: Uuid,street_address: String,city_id: Uuid,state_id: Uuid,country_id: Uuid,address_type_id: Uuid,is_primary: bool,status_id: Uuid) -> Self {
        Self {
            employee_address_id,
employee_id,
street_address,
city_id,
state_id,
country_id,
address_type_id,
is_primary,
status_id

        }
    }
}

impl PartialEq for EmployeeAddresses {
    fn eq(&self, other: &Self) -> bool {
        self.employee_address_id == other.employee_address_id
    }
}

impl Model for EmployeeAddresses {
    fn from_row(row: &PgRow) -> EmployeeAddresses
    where
        Self: Sized,
    {
        let employee_address_id = row.get("EmployeeAddressId");
let employee_id = row.get("EmployeeId");
let street_address = row.get("StreetAddress ");
let city_id = row.get("CityId");
let state_id = row.get("StateId");
let country_id = row.get("CountryId");
let address_type_id = row.get("AddressTypeId");
let is_primary = row.get("IsPrimary");
let status_id = row.get("StatusId");


        Self {
            employee_address_id,
employee_id,
street_address,
city_id,
state_id,
country_id,
address_type_id,
is_primary,
status_id

        }
    }
}
