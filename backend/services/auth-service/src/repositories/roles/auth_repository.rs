use c3k_common::{interfaces::irepository::Model, models::auth::Auth};
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
}
