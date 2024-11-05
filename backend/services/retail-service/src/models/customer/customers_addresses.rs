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
pub struct CustomersAddresses {
    pub customers_address_id: Uuid,
pub address_type_id: Uuid,
pub customer_id: Uuid,
pub address: String,
pub longitude: f64,
pub latitude: f64,
pub status_id: Uuid

}

impl CustomersAddresses {
    pub const TABLE: &'static str = r#""Customer"."CustomersAddresses""#;
    pub const PK: &'static str = "CustomersAddressId";
    pub const COLUMNS_ARRAY: [&'static str; 7] = ["CustomersAddressId","AddressTypeId","CustomerId","Address","Longitude","Latitude","StatusId"];

    pub fn get_id(&self) -> Uuid {
        self.customers_address_id.clone()
    }

    pub fn get_args(&self) -> PgArguments {
        let mut args = PgArguments::default();
        let _ = args.add(self.customers_address_id.clone());
let _ = args.add(self.address_type_id.clone());
let _ = args.add(self.customer_id.clone());
let _ = args.add(self.address.clone());
let _ = args.add(self.longitude.clone());
let _ = args.add(self.latitude.clone());
let _ = args.add(self.status_id.clone());

        args
    }

    pub fn new(customers_address_id: Uuid,address_type_id: Uuid,customer_id: Uuid,address: String,longitude: f64,latitude: f64,status_id: Uuid) -> Self {
        Self {
            customers_address_id,
address_type_id,
customer_id,
address,
longitude,
latitude,
status_id

        }
    }
}

impl PartialEq for CustomersAddresses {
    fn eq(&self, other: &Self) -> bool {
        self.customers_address_id == other.customers_address_id
    }
}

impl Model for CustomersAddresses {
    fn from_row(row: &PgRow) -> CustomersAddresses
    where
        Self: Sized,
    {
        let customers_address_id = row.get("CustomersAddressId");
let address_type_id = row.get("AddressTypeId");
let customer_id = row.get("CustomerId");
let address = row.get("Address");
let longitude = row.get("Longitude");
let latitude = row.get("Latitude");
let status_id = row.get("StatusId");


        Self {
            customers_address_id,
address_type_id,
customer_id,
address,
longitude,
latitude,
status_id

        }
    }
}
