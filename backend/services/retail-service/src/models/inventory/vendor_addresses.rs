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
pub struct VendorAddresses {
    pub vendor_address_id: Uuid,
pub address_type_id: Uuid,
pub vendor_id: Uuid,
pub address: String,
pub longitude: f64,
pub latitude: f64,
pub status_id: Uuid

}

impl VendorAddresses {
    pub const TABLE: &'static str = r#""Inventory"."VendorAddresses""#;
    pub const PK: &'static str = "VendorAddressId";
    pub const COLUMNS_ARRAY: [&'static str; 7] = ["VendorAddressId","AddressTypeId","VendorId","Address","Longitude","Latitude","StatusId"];

    pub fn get_id(&self) -> Uuid {
        self.vendor_address_id.clone()
    }

    pub fn get_args(&self) -> PgArguments {
        let mut args = PgArguments::default();
        let _ = args.add(self.vendor_address_id.clone());
let _ = args.add(self.address_type_id.clone());
let _ = args.add(self.vendor_id.clone());
let _ = args.add(self.address.clone());
let _ = args.add(self.longitude.clone());
let _ = args.add(self.latitude.clone());
let _ = args.add(self.status_id.clone());

        args
    }

    pub fn new(vendor_address_id: Uuid,address_type_id: Uuid,vendor_id: Uuid,address: String,longitude: f64,latitude: f64,status_id: Uuid) -> Self {
        Self {
            vendor_address_id,
address_type_id,
vendor_id,
address,
longitude,
latitude,
status_id

        }
    }
}

impl PartialEq for VendorAddresses {
    fn eq(&self, other: &Self) -> bool {
        self.vendor_address_id == other.vendor_address_id
    }
}

impl Model for VendorAddresses {
    fn from_row(row: &PgRow) -> VendorAddresses
    where
        Self: Sized,
    {
        let vendor_address_id = row.get("VendorAddressId");
let address_type_id = row.get("AddressTypeId");
let vendor_id = row.get("VendorId");
let address = row.get("Address");
let longitude = row.get("Longitude");
let latitude = row.get("Latitude");
let status_id = row.get("StatusId");


        Self {
            vendor_address_id,
address_type_id,
vendor_id,
address,
longitude,
latitude,
status_id

        }
    }
}
