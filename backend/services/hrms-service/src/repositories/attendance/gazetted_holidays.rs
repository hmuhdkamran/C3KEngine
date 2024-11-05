use crate::models::attendance::gazetted_holidays::GazettedHolidays;
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

pub struct GazettedHolidaysRepository {}

impl IRepository<GazettedHolidays> for GazettedHolidaysRepository {
    async fn get_all(connection: PgPool) -> Result<Vec<GazettedHolidays>, Box<dyn StdError>> {
        let result = sqlx::query(&GazettedHolidays::build_select_string(
            GazettedHolidays::TABLE,
            &GazettedHolidays::COLUMNS_ARRAY,
            None,
        ))
        .map(|row: PgRow| GazettedHolidays::from_row(&row))
        .fetch_all(&connection)
        .await
        .map_err(|e| Box::new(e) as Box<dyn StdError>)?;

        Ok(result)
    }

    async fn get_by_filter(
        connection: PgPool,
        filter: &String,
    ) -> Result<Vec<GazettedHolidays>, Box<dyn StdError>> {
        let result = sqlx::query(&GazettedHolidays::build_select_string(
            GazettedHolidays::TABLE,
            &GazettedHolidays::COLUMNS_ARRAY,
            Some(filter),
        ))
        .map(|row: PgRow| GazettedHolidays::from_row(&row))
        .fetch_all(&connection)
        .await
        .map_err(|e| Box::new(e) as Box<dyn StdError>)?;

        Ok(result)
    }

    async fn add(connection: PgPool, entity: &GazettedHolidays) -> Result<bool, Box<dyn StdError>> {
        sqlx::query_with(
            &GazettedHolidays::build_insert_string(GazettedHolidays::TABLE, &GazettedHolidays::COLUMNS_ARRAY),
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

    async fn update(connection: PgPool, entity: &GazettedHolidays) -> Result<bool, Box<dyn StdError>> {
        sqlx::query_with(
            &GazettedHolidays::build_update_string(GazettedHolidays::TABLE, &GazettedHolidays::COLUMNS_ARRAY, GazettedHolidays::PK),
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
            &GazettedHolidays::build_delete_string(GazettedHolidays::TABLE, GazettedHolidays::PK),
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
