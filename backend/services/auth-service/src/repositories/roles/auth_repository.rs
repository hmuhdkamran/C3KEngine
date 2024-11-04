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
    ) -> Result<Vec<Auth>, Box<dyn StdError>> {
        let query = format!(
            r#"SELECT rot."RouteName", rrm."Operation"
FROM "Role"."Users" usr
INNER JOIN "Role"."UserRoleMaps" urm ON usr."UserId" = urm."UserId"
INNER JOIN "Role"."Roles" rol ON urm."RoleId" = rol."RoleId"
INNER JOIN "Role"."RoleRouteMaps" rrm ON rol."RoleId" = rrm."RoleId"
INNER JOIN "Role"."Routes" rot ON rrm."RouteId" = rot."RouteId"
WHERE usr."Username"='{}'"#,
            username
        );
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
            r#"SELECT pr."ProductId", pr."Abbreviation", pr."FullName", pr."Description", pr."Icon" 
FROM "Role"."UserProductMaps" upm
JOIN "Role"."Products" pr ON upm."ProductId" = pr."ProductId"
JOIN "Role"."Users" us ON upm."UserId" = us."UserId"
JOIN "Setup"."Status" st ON upm."StatusId" = st."StatusId" AND st."IsActive" = True
WHERE us."Username"='{}'"#,
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
