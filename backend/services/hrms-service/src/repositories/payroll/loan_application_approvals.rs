use crate::models::payroll::loan_application_approvals::LoanApplicationApprovals;
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

pub struct LoanApplicationApprovalsRepository {}

impl IRepository<LoanApplicationApprovals> for LoanApplicationApprovalsRepository {
    async fn get_all(connection: PgPool) -> Result<Vec<LoanApplicationApprovals>, Box<dyn StdError>> {
        let result = sqlx::query(&LoanApplicationApprovals::build_select_string(
            LoanApplicationApprovals::TABLE,
            &LoanApplicationApprovals::COLUMNS_ARRAY,
            None,
        ))
        .map(|row: PgRow| LoanApplicationApprovals::from_row(&row))
        .fetch_all(&connection)
        .await
        .map_err(|e| Box::new(e) as Box<dyn StdError>)?;

        Ok(result)
    }

    async fn get_by_filter(
        connection: PgPool,
        filter: &String,
    ) -> Result<Vec<LoanApplicationApprovals>, Box<dyn StdError>> {
        let result = sqlx::query(&LoanApplicationApprovals::build_select_string(
            LoanApplicationApprovals::TABLE,
            &LoanApplicationApprovals::COLUMNS_ARRAY,
            Some(filter),
        ))
        .map(|row: PgRow| LoanApplicationApprovals::from_row(&row))
        .fetch_all(&connection)
        .await
        .map_err(|e| Box::new(e) as Box<dyn StdError>)?;

        Ok(result)
    }

    async fn add(connection: PgPool, entity: &LoanApplicationApprovals) -> Result<bool, Box<dyn StdError>> {
        sqlx::query_with(
            &LoanApplicationApprovals::build_insert_string(LoanApplicationApprovals::TABLE, &LoanApplicationApprovals::COLUMNS_ARRAY),
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

    async fn update(connection: PgPool, entity: &LoanApplicationApprovals) -> Result<bool, Box<dyn StdError>> {
        sqlx::query_with(
            &LoanApplicationApprovals::build_update_string(LoanApplicationApprovals::TABLE, &LoanApplicationApprovals::COLUMNS_ARRAY, LoanApplicationApprovals::PK),
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
            &LoanApplicationApprovals::build_delete_string(LoanApplicationApprovals::TABLE, LoanApplicationApprovals::PK),
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
