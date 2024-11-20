use c3k_common::{
    interfaces::irepository::Model,
    models::auth::{Auth, UserProducts},
};
pub use sqlx::{
    pool::PoolConnection,
    postgres::{PgArguments, PgPoolOptions, PgRow},
    Arguments, PgPool, Postgres, Row,
};
use std::error::Error as StdError;

pub struct AuthRepository {}

impl AuthRepository {
    pub async fn get_claims(
        connection: PgPool,
        username: &String,
        product: Option<String>, // Optional parameter for the product
    ) -> Result<Vec<Auth>, Box<dyn StdError>> {
        // Build the query based on whether product is provided
        let query = if let Some(ref p) = product {
            // If product is provided, use UserApplicationRoles
            format!(
                r#"SELECT uar."RouteName", uar."Operation" 
                   FROM "Role"."UserApplicationRoles" uar 
                   WHERE uar."Username"='{}' AND uar."Product"='{}'"#,
                username, p
            )
        } else {
            // If product is not provided, use UserRoles
            format!(
                r#"SELECT ur."RouteName", ur."Operation" 
                   FROM "Role"."UserRoles" ur 
                   WHERE ur."Username"='{}'"#,
                username
            )
        };

        // Execute the query
        let result = sqlx::query(query.as_str())
            .map(|row: PgRow| Auth::from_row(&row))
            .fetch_all(&connection)
            .await
            .map_err(|e| Box::new(e) as Box<dyn StdError>)?;

        Ok(result)
    }

    pub async fn get_products(
        connection: PgPool,
        username: &String,
    ) -> Result<Vec<UserProducts>, Box<dyn StdError>> {
        let query = format!(
            r#"SELECT pr."ProductId", pr."Abbreviation", pr."FullName", pr."Description", pr."Icon", pr."FrontendIp", pr."FrontendPort" FROM "Role"."UserApplications" pr WHERE us."Username"='{}'"#,
            username
        );
        let result = sqlx::query(query.as_str())
            .map(|row: PgRow| UserProducts::from_row(&row))
            .fetch_all(&connection)
            .await
            .map_err(|e| Box::new(e) as Box<dyn StdError>)?;

        Ok(result)
    }
}
