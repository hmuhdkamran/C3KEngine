pub use sqlx::{
    pool::PoolConnection,
    postgres::{PgArguments, PgPoolOptions, PgRow},
    Arguments, PgPool, Postgres, Row,
};
use std::error::Error as StdError;

use crate::{
    interfaces::irepository::{IRepository, Model},
    models::{
        constants::{
            MESSAGE_CAN_NOT_DELETE_DATA, MESSAGE_CAN_NOT_INSERT_DATA, MESSAGE_CAN_NOT_UPDATE_DATA,
        },
        roles::role::Role,
    },
    utilities::error_display::ParseError
};

pub struct RoleRepository {}

impl IRepository<Role> for RoleRepository {
    async fn get_all(connection: PgPool) -> Result<Vec<Role>, Box<dyn StdError>> {
        let result = sqlx::query(format!("SELECT {} FROM {}", Role::COLUMNS, Role::TABLE).as_str())
            .map(|row: PgRow| Role::from_row(&row))
            .fetch_all(&connection)
            .await
            .map_err(|e| Box::new(e) as Box<dyn StdError>)?;

        Ok(result)
    }

    async fn get_by_filter(
        connection: PgPool,
        filter: &String,
    ) -> Result<Vec<Role>, Box<dyn StdError>> {
        let query = format!(
            r#"SELECT {} FROM {} WHERE {}"#,
            Role::COLUMNS,
            Role::TABLE,
            filter
        );
        let result = sqlx::query(query.as_str())
            .map(|row: PgRow| Role::from_row(&row))
            .fetch_all(&connection)
            .await
            .map_err(|e| Box::new(e) as Box<dyn StdError>)?;

        Ok(result)
    }

    async fn add(connection: PgPool, entity: &Role) -> Result<bool, Box<dyn StdError>> {
        let mut args = PgArguments::default();
        args.add(entity.role_id.clone());
        args.add(entity.parent_role_id.clone());
        args.add(entity.full_name.clone());
        args.add(entity.status_id.clone());

        sqlx::query_with(
            format!(
                "INSERT INTO {} ({}) VALUES ($1, $2, $3, $4)",
                Role::TABLE,
                Role::COLUMNS
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

    async fn update(connection: PgPool, entity: &Role) -> Result<bool, Box<dyn StdError>> {
        let mut args = PgArguments::default();
        args.add(entity.role_id.clone());
        args.add(entity.parent_role_id.clone());
        args.add(entity.full_name.clone());
        args.add(entity.status_id.clone());

        sqlx::query_with(
            format!("UPDATE {} SET {}", Role::TABLE, Role::COLUMNS_UPDATE).as_str(),
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
        args.add(id);

        sqlx::query_with(
            format!("DELETE FROM {} WHERE {}", Role::TABLE, Role::PK).as_str(),
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
