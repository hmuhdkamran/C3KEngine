use crate::models::payroll::salary_deductions::SalaryDeductions;
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

pub struct SalaryDeductionsRepository {}

impl IRepository<SalaryDeductions> for SalaryDeductionsRepository {
    async fn get_all(connection: PgPool) -> Result<Vec<SalaryDeductions>, Box<dyn StdError>> {
        let result = sqlx::query(&SalaryDeductions::build_select_string(
            SalaryDeductions::TABLE,
            &SalaryDeductions::COLUMNS_ARRAY,
            None,
        ))
        .map(|row: PgRow| SalaryDeductions::from_row(&row))
        .fetch_all(&connection)
        .await
        .map_err(|e| Box::new(e) as Box<dyn StdError>)?;

        Ok(result)
    }

    async fn get_by_filter(
        connection: PgPool,
        filter: &String,
    ) -> Result<Vec<SalaryDeductions>, Box<dyn StdError>> {
        let result = sqlx::query(&SalaryDeductions::build_select_string(
            SalaryDeductions::TABLE,
            &SalaryDeductions::COLUMNS_ARRAY,
            Some(filter),
        ))
        .map(|row: PgRow| SalaryDeductions::from_row(&row))
        .fetch_all(&connection)
        .await
        .map_err(|e| Box::new(e) as Box<dyn StdError>)?;

        Ok(result)
    }

    async fn add(connection: PgPool, entity: &SalaryDeductions) -> Result<bool, Box<dyn StdError>> {
        sqlx::query_with(
            &SalaryDeductions::build_insert_string(SalaryDeductions::TABLE, &SalaryDeductions::COLUMNS_ARRAY),
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

    async fn update(connection: PgPool, entity: &SalaryDeductions) -> Result<bool, Box<dyn StdError>> {
        sqlx::query_with(
            &SalaryDeductions::build_update_string(SalaryDeductions::TABLE, &SalaryDeductions::COLUMNS_ARRAY, SalaryDeductions::PK),
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
            &SalaryDeductions::build_delete_string(SalaryDeductions::TABLE, SalaryDeductions::PK),
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
