use c3k_common::interfaces::irepository::{IRepository, Model};
pub use sqlx::{
    pool::PoolConnection,
    postgres::{PgArguments, PgPoolOptions, PgRow},
    Arguments, PgPool, Postgres, Row,
};
use std::error::Error as StdError;

use crate::{
    models::{
        constants::{
            MESSAGE_CAN_NOT_DELETE_DATA, MESSAGE_CAN_NOT_INSERT_DATA, MESSAGE_CAN_NOT_UPDATE_DATA,
        },
        roles::user_role_map::UserRoleMap,
    },
    utilities::error_display::ParseError,
};

pub struct UserRoleMapRepository {}

impl IRepository<UserRoleMap> for UserRoleMapRepository {
    async fn get_all(connection: PgPool) -> Result<Vec<UserRoleMap>, Box<dyn StdError>> {
        let result = sqlx::query(
            format!(
                "SELECT {} FROM {}",
                UserRoleMap::COLUMNS,
                UserRoleMap::TABLE
            )
            .as_str(),
        )
        .map(|row: PgRow| UserRoleMap::from_row(&row))
        .fetch_all(&connection)
        .await
        .map_err(|e| Box::new(e) as Box<dyn StdError>)?;

        Ok(result)
    }

    async fn get_by_filter(
        connection: PgPool,
        filter: &String,
    ) -> Result<Vec<UserRoleMap>, Box<dyn StdError>> {
        let query = format!(
            r#"SELECT {} FROM {} WHERE {}"#,
            UserRoleMap::COLUMNS,
            UserRoleMap::TABLE,
            filter
        );
        let result = sqlx::query(query.as_str())
            .map(|row: PgRow| UserRoleMap::from_row(&row))
            .fetch_all(&connection)
            .await
            .map_err(|e| Box::new(e) as Box<dyn StdError>)?;

        Ok(result)
    }

    async fn add(connection: PgPool, entity: &UserRoleMap) -> Result<bool, Box<dyn StdError>> {
        let mut args = PgArguments::default();
        let _ = args.add(entity.user_role_map_id.clone());
        let _ = args.add(entity.role_id.clone());
        let _ = args.add(entity.user_id.clone());
        let _ = args.add(entity.status_id.clone());

        sqlx::query_with(
            format!(
                "INSERT INTO {} ({}) VALUES ($1, $2, $3, $4)",
                UserRoleMap::TABLE,
                UserRoleMap::COLUMNS
            )
            .as_str(),
            args,
        )
        .execute(&connection)
        .await
        .map(|result| {
            if result.rows_affected() > 0 {
                Ok(true)
            } else {
                Err(Box::new(ParseError::from(MESSAGE_CAN_NOT_INSERT_DATA)) as Box<dyn StdError>)
            }
        })
        .map_err(|e| Box::new(e) as Box<dyn StdError>)
        .unwrap_or_else(|e| Err(e))
    }

    async fn update(connection: PgPool, entity: &UserRoleMap) -> Result<bool, Box<dyn StdError>> {
        let mut args = PgArguments::default();
        let _ = args.add(entity.user_role_map_id.clone());
        let _ = args.add(entity.role_id.clone());
        let _ = args.add(entity.user_id.clone());
        let _ = args.add(entity.status_id.clone());

        sqlx::query_with(
            format!(
                "UPDATE {} SET {}",
                UserRoleMap::TABLE,
                UserRoleMap::COLUMNS_UPDATE
            )
            .as_str(),
            args,
        )
        .execute(&connection)
        .await
        .map(|result| {
            if result.rows_affected() > 0 {
                Ok(true)
            } else {
                Err(Box::new(ParseError::from(MESSAGE_CAN_NOT_UPDATE_DATA)) as Box<dyn StdError>)
            }
        })
        .map_err(|e| Box::new(e) as Box<dyn StdError>)
        .unwrap_or_else(|e| Err(e))
    }

    async fn delete(connection: PgPool, id: &String) -> Result<bool, Box<dyn StdError>> {
        let mut args = PgArguments::default();
        let _ = args.add(id);

        sqlx::query_with(
            format!(
                "DELETE FROM {} WHERE {}",
                UserRoleMap::TABLE,
                UserRoleMap::PK
            )
            .as_str(),
            args,
        )
        .execute(&connection)
        .await
        .map(|result| {
            if result.rows_affected() > 0 {
                Ok(true)
            } else {
                Err(Box::new(ParseError::from(MESSAGE_CAN_NOT_DELETE_DATA)) as Box<dyn StdError>)
            }
        })
        .map_err(|e| Box::new(e) as Box<dyn StdError>)
        .unwrap_or_else(|e| Err(e))
    }
}
