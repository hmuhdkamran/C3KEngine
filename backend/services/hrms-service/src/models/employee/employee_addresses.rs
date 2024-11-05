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
    pub const PK: &'static str = "EmployeeAddressId";
    pub const COLUMNS_ARRAY: [&'static str; 9] = ["EmployeeAddressId","EmployeeId","StreetAddress ","CityId","StateId","CountryId","AddressTypeId","IsPrimary","StatusId"];

    pub fn get_id(&self) -> Uuid {
        self.employee_address_id.clone()
    }

    pub fn get_args(&self) -> PgArguments {
        let mut args = PgArguments::default();
        let _ = args.add(self.employee_address_id.clone());
let _ = args.add(self.employee_id.clone());
let _ = args.add(self.street_address.clone());
let _ = args.add(self.city_id.clone());
let _ = args.add(self.state_id.clone());
let _ = args.add(self.country_id.clone());
let _ = args.add(self.address_type_id.clone());
let _ = args.add(self.is_primary.clone());
let _ = args.add(self.status_id.clone());

        args
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
