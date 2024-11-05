use crate::models::setup::ownership_status::OwnershipStatus;
use c3k_common::{
    handler::error_display::ParseError,
    interfaces::irepository::{IRepository, Model},
    models::constants::{
        MESSAGE_CAN_NOT_DELETE_DATA, MESSAGE_CAN_NOT_INSERT_DATA, MESSAGE_CAN_NOT_UPDATE_DATA,
    },
};
pub use sqlx::{
    pool::PoolConnection,
    postgres::{PgArguments, PgPoolOptions, PgRow},
    Arguments, PgPool, Postgres, Row,
};
use std::error::Error as StdError;

pub struct OwnershipStatusRepository {}

impl IRepository<OwnershipStatus> for OwnershipStatusRepository {
    async fn get_all(connection: PgPool) -> Result<Vec<OwnershipStatus>, Box<dyn StdError>> {
        let result = sqlx::query(&OwnershipStatus::build_select_string(
            OwnershipStatus::TABLE,
            &OwnershipStatus::COLUMNS_ARRAY,
            None,
        ))
        .map(|row: PgRow| OwnershipStatus::from_row(&row))
        .fetch_all(&connection)
        .await
        .map_err(|e| Box::new(e) as Box<dyn StdError>)?;

        Ok(result)
    }

    async fn get_by_filter(
        connection: PgPool,
        filter: &String,
    ) -> Result<Vec<OwnershipStatus>, Box<dyn StdError>> {
        let result = sqlx::query(&OwnershipStatus::build_select_string(
            OwnershipStatus::TABLE,
            &OwnershipStatus::COLUMNS_ARRAY,
            Some(filter),
        ))
        .map(|row: PgRow| OwnershipStatus::from_row(&row))
        .fetch_all(&connection)
        .await
        .map_err(|e| Box::new(e) as Box<dyn StdError>)?;

        Ok(result)
    }

    async fn add(connection: PgPool, entity: &OwnershipStatus) -> Result<bool, Box<dyn StdError>> {
        sqlx::query_with(
            &OwnershipStatus::build_insert_string(OwnershipStatus::TABLE, &OwnershipStatus::COLUMNS_ARRAY),
            entity.get_args(),
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

    async fn update(connection: PgPool, entity: &OwnershipStatus) -> Result<bool, Box<dyn StdError>> {
        sqlx::query_with(
            &OwnershipStatus::build_update_string(OwnershipStatus::TABLE, &OwnershipStatus::COLUMNS_ARRAY, OwnershipStatus::PK),
            entity.get_args(),
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
            &OwnershipStatus::build_delete_string(OwnershipStatus::TABLE, OwnershipStatus::PK),
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
