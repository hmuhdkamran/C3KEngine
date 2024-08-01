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
        roles::role_route_map::RoleRouteMap,
    },
    utilities::error_display::ParseError
};

pub struct RoleRouteMapRepository {}

impl IRepository<RoleRouteMap> for RoleRouteMapRepository {
    async fn get_all(connection: PgPool) -> Result<Vec<RoleRouteMap>, Box<dyn StdError>> {
        let result = sqlx::query(format!("SELECT {} FROM {}", RoleRouteMap::COLUMNS, RoleRouteMap::TABLE).as_str())
            .map(|row: PgRow| RoleRouteMap::from_row(&row))
            .fetch_all(&connection)
            .await
            .map_err(|e| Box::new(e) as Box<dyn StdError>)?;

        Ok(result)
    }

    async fn get_by_filter(
        connection: PgPool,
        filter: &String,
    ) -> Result<Vec<RoleRouteMap>, Box<dyn StdError>> {
        let query = format!(
            r#"SELECT {} FROM {} WHERE {}"#,
            RoleRouteMap::COLUMNS,
            RoleRouteMap::TABLE,
            filter
        );
        let result = sqlx::query(query.as_str())
            .map(|row: PgRow| RoleRouteMap::from_row(&row))
            .fetch_all(&connection)
            .await
            .map_err(|e| Box::new(e) as Box<dyn StdError>)?;

        Ok(result)
    }

    async fn add(connection: PgPool, entity: &RoleRouteMap) -> Result<bool, Box<dyn StdError>> {
        let mut args = PgArguments::default();
        let _ = args.add(entity.role_route_id.clone());
        let _ = args.add(entity.role_id.clone());
        let _ = args.add(entity.route_id.clone());
        let _ = args.add(entity.operation.clone());
        let _ = args.add(entity.status_id.clone());

        sqlx::query_with(
            format!(
                "INSERT INTO {} ({}) VALUES ($1, $2, $3, $4, $5)",
                RoleRouteMap::TABLE,
                RoleRouteMap::COLUMNS
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

    async fn update(connection: PgPool, entity: &RoleRouteMap) -> Result<bool, Box<dyn StdError>> {
        let mut args = PgArguments::default();
        let _ = args.add(entity.role_route_id.clone());
        let _ = args.add(entity.role_id.clone());
        let _ = args.add(entity.route_id.clone());
        let _ = args.add(entity.operation.clone());
        let _ = args.add(entity.status_id.clone());

        sqlx::query_with(
            format!("UPDATE {} SET {}", RoleRouteMap::TABLE, RoleRouteMap::COLUMNS_UPDATE).as_str(),
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
            format!("DELETE FROM {} WHERE {}", RoleRouteMap::TABLE, RoleRouteMap::PK).as_str(),
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
