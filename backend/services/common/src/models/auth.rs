use serde::{Deserialize, Serialize};
use sqlx::types::Uuid;
pub use sqlx::{
    pool::PoolConnection,
    postgres::{PgArguments, PgPoolOptions, PgRow},
    Arguments, Error, PgPool, Postgres, Row,
};

use crate::interfaces::irepository::Model;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Auth {
    pub route: String,
    pub allow: String,
}

impl Auth {
    pub fn new(route: String, allow: String) -> Self {
        Self { route, allow }
    }
}

impl PartialEq for Auth {
    fn eq(&self, other: &Self) -> bool {
        self.route == other.allow
    }
}

impl Model for Auth {
    fn from_row(row: &PgRow) -> Auth
    where
        Self: Sized,
    {
        let route = row.get("RouteName");
        let allow = row.get("Operation");

        Self { route, allow }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JwtClaims {
    pub aud: String,
    pub expiry: u64,
    #[serde(rename = "http://schemas.xmlsoap.org/ws/2005/05/identity/claims/sid")]
    pub sid: String,
    #[serde(rename = "http://schemas.xmlsoap.org/ws/2005/05/identity/claims/emailaddress")]
    pub emailaddress: String,
    #[serde(rename = "http://schemas.xmlsoap.org/ws/2005/05/identity/claims/name")]
    pub name: Vec<String>,
    #[serde(rename = "http://schemas.xmlsoap.org/ws/2005/05/identity/claims/role")]
    pub role: Vec<Auth>,
    #[serde(rename = "http://schemas.c3kframework.skn/2003/81/culturename")]
    pub culturename: String,
    pub iss: String,
    pub sub: String,
    pub typ: String,
    pub exp: u64,
    pub iat: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PasswordCode {
    pub salt: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthModel {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserProducts {
    pub product_id: Uuid,
    pub abbreviation: String,
    pub full_name: String,
    pub description: String,
    pub icon: String,
}

impl UserProducts {
    pub fn new(
        product_id: Uuid,
        abbreviation: String,
        full_name: String,
        description: String,
        icon: String,
    ) -> Self {
        Self {
            product_id,
            abbreviation,
            full_name,
            description,
            icon,
        }
    }
}

impl PartialEq for UserProducts {
    fn eq(&self, other: &Self) -> bool {
        self.product_id == other.product_id
    }
}

impl Model for UserProducts {
    fn from_row(row: &PgRow) -> UserProducts
    where
        Self: Sized,
    {
        let product_id = row.get("ProductId");
        let abbreviation = row.get("Abbreviation");
        let full_name = row.get("FullName");
        let description = row.get("Description");
        let icon = row.get("Icon");

        Self {
            product_id,
            abbreviation,
            full_name,
            description,
            icon,
        }
    }
}
