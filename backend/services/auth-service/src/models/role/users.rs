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
pub struct Users {
    pub user_id: Uuid,
pub username: String,
pub display_name: String,
pub language: String,
pub password: String,
pub salt: String,
pub status_id: Uuid

}

impl Users {
    pub const TABLE: &'static str = r#""Role"."Users""#;
    pub const PK: &'static str = "UserId";
    pub const COLUMNS_ARRAY: [&'static str; 7] = ["UserId","Username","DisplayName","Language","Password","Salt","StatusId"];

    pub fn get_id(&self) -> Uuid {
        self.user_id.clone()
    }

    pub fn get_args(&self) -> PgArguments {
        let mut args = PgArguments::default();
        let _ = args.add(self.user_id.clone());
let _ = args.add(self.username.clone());
let _ = args.add(self.display_name.clone());
let _ = args.add(self.language.clone());
let _ = args.add(self.password.clone());
let _ = args.add(self.salt.clone());
let _ = args.add(self.status_id.clone());

        args
    }

    pub fn new(user_id: Uuid,username: String,display_name: String,language: String,password: String,salt: String,status_id: Uuid) -> Self {
        Self {
            user_id,
username,
display_name,
language,
password,
salt,
status_id

        }
    }
}

impl PartialEq for Users {
    fn eq(&self, other: &Self) -> bool {
        self.user_id == other.user_id
    }
}

impl Model for Users {
    fn from_row(row: &PgRow) -> Users
    where
        Self: Sized,
    {
        let user_id = row.get("UserId");
let username = row.get("Username");
let display_name = row.get("DisplayName");
let language = row.get("Language");
let password = row.get("Password");
let salt = row.get("Salt");
let status_id = row.get("StatusId");


        Self {
            user_id,
username,
display_name,
language,
password,
salt,
status_id

        }
    }
}
