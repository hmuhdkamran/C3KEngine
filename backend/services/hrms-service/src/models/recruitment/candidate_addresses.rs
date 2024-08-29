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
pub struct CandidateAddresses {
    pub candidate_address_id: Uuid,
pub address_type_id: Uuid,
pub street_address: String,
pub city_id: Uuid,
pub is_primary: bool,
pub status_id: Uuid,
pub candidate_id: Uuid

}

impl CandidateAddresses {
    pub const TABLE: &'static str = r#""Recruitment"."CandidateAddresses""#;
    pub const PK: &'static str = r#"CandidateAddressId::TEXT=$1"#;
    pub const COLUMNS: &'static str = r#""CandidateAddressId","AddressTypeId","StreetAddress","CityId","IsPrimary","StatusId","CandidateId""#;
    pub const COLUMNS_UPDATE: &'static str = r#""CandidateAddressId"=$1,"AddressTypeId"=$2,"StreetAddress"=$3,"CityId"=$4,"IsPrimary"=$7,"StatusId"=$8,"CandidateId"=$9 WHERE "CandidateAddressId"=$1"#;

    pub fn get_id(&self) -> Uuid {
        self.candidate_address_id.clone()
    }

    pub fn new(candidate_address_id: Uuid,address_type_id: Uuid,street_address: String,city_id: Uuid,is_primary: bool,status_id: Uuid,candidate_id: Uuid) -> Self {
        Self {
            candidate_address_id,
address_type_id,
street_address,
city_id,
is_primary,
status_id,
candidate_id

        }
    }
}

impl PartialEq for CandidateAddresses {
    fn eq(&self, other: &Self) -> bool {
        self.candidate_address_id == other.candidate_address_id
    }
}

impl Model for CandidateAddresses {
    fn from_row(row: &PgRow) -> CandidateAddresses
    where
        Self: Sized,
    {
        let candidate_address_id = row.get("CandidateAddressId");
let address_type_id = row.get("AddressTypeId");
let street_address = row.get("StreetAddress");
let city_id = row.get("CityId");
let is_primary = row.get("IsPrimary");
let status_id = row.get("StatusId");
let candidate_id = row.get("CandidateId");


        Self {
            candidate_address_id,
address_type_id,
street_address,
city_id,
is_primary,
status_id,
candidate_id

        }
    }
}
