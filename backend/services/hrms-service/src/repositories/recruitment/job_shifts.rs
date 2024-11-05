use crate::models::recruitment::job_shifts::JobShifts;
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

pub struct JobShiftsRepository {}

impl IRepository<JobShifts> for JobShiftsRepository {
    async fn get_all(connection: PgPool) -> Result<Vec<JobShifts>, Box<dyn StdError>> {
        let result = sqlx::query(&JobShifts::build_select_string(
            JobShifts::TABLE,
            &JobShifts::COLUMNS_ARRAY,
            None,
        ))
        .map(|row: PgRow| JobShifts::from_row(&row))
        .fetch_all(&connection)
        .await
        .map_err(|e| Box::new(e) as Box<dyn StdError>)?;

        Ok(result)
    }

    async fn get_by_filter(
        connection: PgPool,
        filter: &String,
    ) -> Result<Vec<JobShifts>, Box<dyn StdError>> {
        let result = sqlx::query(&JobShifts::build_select_string(
            JobShifts::TABLE,
            &JobShifts::COLUMNS_ARRAY,
            Some(filter),
        ))
        .map(|row: PgRow| JobShifts::from_row(&row))
        .fetch_all(&connection)
        .await
        .map_err(|e| Box::new(e) as Box<dyn StdError>)?;

        Ok(result)
    }

    async fn add(connection: PgPool, entity: &JobShifts) -> Result<bool, Box<dyn StdError>> {
        sqlx::query_with(
            &JobShifts::build_insert_string(JobShifts::TABLE, &JobShifts::COLUMNS_ARRAY),
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

    async fn update(connection: PgPool, entity: &JobShifts) -> Result<bool, Box<dyn StdError>> {
        sqlx::query_with(
            &JobShifts::build_update_string(JobShifts::TABLE, &JobShifts::COLUMNS_ARRAY, JobShifts::PK),
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
            &JobShifts::build_delete_string(JobShifts::TABLE, JobShifts::PK),
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
