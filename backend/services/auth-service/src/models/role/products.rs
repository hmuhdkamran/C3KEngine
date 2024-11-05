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
pub struct Products {
    pub product_id: Uuid,
pub abbreviation: String,
pub full_name: String,
pub description: String,
pub icon: String,
pub api_prefix: String,
pub connection_string: String,
pub host_ip: String,
pub host_port: i32,
pub frontend_ip: String,
pub frontend_port: i32,
pub status_id: Uuid

}

impl Products {
    pub const TABLE: &'static str = r#""Role"."Products""#;
    pub const PK: &'static str = "ProductId";
    pub const COLUMNS_ARRAY: [&'static str; 12] = ["ProductId","Abbreviation","FullName","Description","Icon","ApiPrefix","ConnectionString","HostIp","HostPort","FrontendIp","FrontendPort","StatusId"];

    pub fn get_id(&self) -> Uuid {
        self.product_id.clone()
    }

    pub fn get_args(&self) -> PgArguments {
        let mut args = PgArguments::default();
        let _ = args.add(self.product_id.clone());
let _ = args.add(self.abbreviation.clone());
let _ = args.add(self.full_name.clone());
let _ = args.add(self.description.clone());
let _ = args.add(self.icon.clone());
let _ = args.add(self.api_prefix.clone());
let _ = args.add(self.connection_string.clone());
let _ = args.add(self.host_ip.clone());
let _ = args.add(self.host_port.clone());
let _ = args.add(self.frontend_ip.clone());
let _ = args.add(self.frontend_port.clone());
let _ = args.add(self.status_id.clone());

        args
    }

    pub fn new(product_id: Uuid,abbreviation: String,full_name: String,description: String,icon: String,api_prefix: String,connection_string: String,host_ip: String,host_port: i32,frontend_ip: String,frontend_port: i32,status_id: Uuid) -> Self {
        Self {
            product_id,
abbreviation,
full_name,
description,
icon,
api_prefix,
connection_string,
host_ip,
host_port,
frontend_ip,
frontend_port,
status_id

        }
    }
}

impl PartialEq for Products {
    fn eq(&self, other: &Self) -> bool {
        self.product_id == other.product_id
    }
}

impl Model for Products {
    fn from_row(row: &PgRow) -> Products
    where
        Self: Sized,
    {
        let product_id = row.get("ProductId");
let abbreviation = row.get("Abbreviation");
let full_name = row.get("FullName");
let description = row.get("Description");
let icon = row.get("Icon");
let api_prefix = row.get("ApiPrefix");
let connection_string = row.get("ConnectionString");
let host_ip = row.get("HostIp");
let host_port = row.get("HostPort");
let frontend_ip = row.get("FrontendIp");
let frontend_port = row.get("FrontendPort");
let status_id = row.get("StatusId");


        Self {
            product_id,
abbreviation,
full_name,
description,
icon,
api_prefix,
connection_string,
host_ip,
host_port,
frontend_ip,
frontend_port,
status_id

        }
    }
}
