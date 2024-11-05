use crate::models::payroll::travel_request_approvals::TravelRequestApprovals;
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

pub struct TravelRequestApprovalsRepository {}

impl IRepository<TravelRequestApprovals> for TravelRequestApprovalsRepository {
    async fn get_all(connection: PgPool) -> Result<Vec<TravelRequestApprovals>, Box<dyn StdError>> {
        let result = sqlx::query(&TravelRequestApprovals::build_select_string(
            TravelRequestApprovals::TABLE,
            &TravelRequestApprovals::COLUMNS_ARRAY,
            None,
        ))
        .map(|row: PgRow| TravelRequestApprovals::from_row(&row))
        .fetch_all(&connection)
        .await
        .map_err(|e| Box::new(e) as Box<dyn StdError>)?;

        Ok(result)
    }

    async fn get_by_filter(
        connection: PgPool,
        filter: &String,
    ) -> Result<Vec<TravelRequestApprovals>, Box<dyn StdError>> {
        let result = sqlx::query(&TravelRequestApprovals::build_select_string(
            TravelRequestApprovals::TABLE,
            &TravelRequestApprovals::COLUMNS_ARRAY,
            Some(filter),
        ))
        .map(|row: PgRow| TravelRequestApprovals::from_row(&row))
        .fetch_all(&connection)
        .await
        .map_err(|e| Box::new(e) as Box<dyn StdError>)?;

        Ok(result)
    }

    async fn add(connection: PgPool, entity: &TravelRequestApprovals) -> Result<bool, Box<dyn StdError>> {
        sqlx::query_with(
            &TravelRequestApprovals::build_insert_string(TravelRequestApprovals::TABLE, &TravelRequestApprovals::COLUMNS_ARRAY),
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

    async fn update(connection: PgPool, entity: &TravelRequestApprovals) -> Result<bool, Box<dyn StdError>> {
        sqlx::query_with(
            &TravelRequestApprovals::build_update_string(TravelRequestApprovals::TABLE, &TravelRequestApprovals::COLUMNS_ARRAY, TravelRequestApprovals::PK),
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
            &TravelRequestApprovals::build_delete_string(TravelRequestApprovals::TABLE, TravelRequestApprovals::PK),
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
