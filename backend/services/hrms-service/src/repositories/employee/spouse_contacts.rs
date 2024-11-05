use crate::models::employee::spouse_contacts::SpouseContacts;
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

pub struct SpouseContactsRepository {}

impl IRepository<SpouseContacts> for SpouseContactsRepository {
    async fn get_all(connection: PgPool) -> Result<Vec<SpouseContacts>, Box<dyn StdError>> {
        let result = sqlx::query(&SpouseContacts::build_select_string(
            SpouseContacts::TABLE,
            &SpouseContacts::COLUMNS_ARRAY,
            None,
        ))
        .map(|row: PgRow| SpouseContacts::from_row(&row))
        .fetch_all(&connection)
        .await
        .map_err(|e| Box::new(e) as Box<dyn StdError>)?;

        Ok(result)
    }

    async fn get_by_filter(
        connection: PgPool,
        filter: &String,
    ) -> Result<Vec<SpouseContacts>, Box<dyn StdError>> {
        let result = sqlx::query(&SpouseContacts::build_select_string(
            SpouseContacts::TABLE,
            &SpouseContacts::COLUMNS_ARRAY,
            Some(filter),
        ))
        .map(|row: PgRow| SpouseContacts::from_row(&row))
        .fetch_all(&connection)
        .await
        .map_err(|e| Box::new(e) as Box<dyn StdError>)?;

        Ok(result)
    }

    async fn add(connection: PgPool, entity: &SpouseContacts) -> Result<bool, Box<dyn StdError>> {
        sqlx::query_with(
            &SpouseContacts::build_insert_string(SpouseContacts::TABLE, &SpouseContacts::COLUMNS_ARRAY),
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

    async fn update(connection: PgPool, entity: &SpouseContacts) -> Result<bool, Box<dyn StdError>> {
        sqlx::query_with(
            &SpouseContacts::build_update_string(SpouseContacts::TABLE, &SpouseContacts::COLUMNS_ARRAY, SpouseContacts::PK),
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
            &SpouseContacts::build_delete_string(SpouseContacts::TABLE, SpouseContacts::PK),
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
