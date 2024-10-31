use c3k_common::interfaces::irepository::Model;
use serde::{Deserialize, Serialize};
pub use sqlx::{
    pool::PoolConnection,
    postgres::{PgArguments, PgPoolOptions, PgRow},
    Arguments, Error, PgPool, Postgres, Row,
};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct User {
    pub user_id: Uuid,
    pub username: String,
    pub display_name: String,
    pub language: String,
    pub password: String,
    pub salt: String,
    pub status_id: Uuid,
}

impl User {
    pub const TABLE: &'static str = r#""Role"."Users""#;
    pub const PK: &'static str = r#""UserId"::TEXT=$1"#;
    pub const COLUMNS: &'static str =
        r#""UserId", "Username", "DisplayName", "Language", "Password", "Salt", "StatusId""#;
    pub const COLUMNS_UPDATE: &'static str = r#""Username"=$2, "DisplayName"=$3, "Language"=$4, "Password"=$5, "Salt"=$6, "StatusId"=$7 WHERE "UserId"=$1"#;

    pub fn get_id(&self) -> Uuid {
        self.user_id.clone()
    }

    pub fn new(
        user_id: Uuid,
        username: String,
        display_name: String,
        language: String,
        password: String,
        salt: String,
        status_id: Uuid,
    ) -> Self {
        Self {
            user_id,
            username,
            display_name,
            language,
            password,
            salt,
            status_id,
        }
    }
}

impl PartialEq for User {
    fn eq(&self, other: &Self) -> bool {
        self.user_id == other.user_id
    }
}

impl Model for User {
    fn from_row(row: &PgRow) -> User
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
            status_id,
        }
    }
}
