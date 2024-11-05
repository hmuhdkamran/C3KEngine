use crate::models::payroll::employee_expenses::EmployeeExpenses;
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

pub struct EmployeeExpensesRepository {}

impl IRepository<EmployeeExpenses> for EmployeeExpensesRepository {
    async fn get_all(connection: PgPool) -> Result<Vec<EmployeeExpenses>, Box<dyn StdError>> {
        let result = sqlx::query(&EmployeeExpenses::build_select_string(
            EmployeeExpenses::TABLE,
            &EmployeeExpenses::COLUMNS_ARRAY,
            None,
        ))
        .map(|row: PgRow| EmployeeExpenses::from_row(&row))
        .fetch_all(&connection)
        .await
        .map_err(|e| Box::new(e) as Box<dyn StdError>)?;

        Ok(result)
    }

    async fn get_by_filter(
        connection: PgPool,
        filter: &String,
    ) -> Result<Vec<EmployeeExpenses>, Box<dyn StdError>> {
        let result = sqlx::query(&EmployeeExpenses::build_select_string(
            EmployeeExpenses::TABLE,
            &EmployeeExpenses::COLUMNS_ARRAY,
            Some(filter),
        ))
        .map(|row: PgRow| EmployeeExpenses::from_row(&row))
        .fetch_all(&connection)
        .await
        .map_err(|e| Box::new(e) as Box<dyn StdError>)?;

        Ok(result)
    }

    async fn add(connection: PgPool, entity: &EmployeeExpenses) -> Result<bool, Box<dyn StdError>> {
        sqlx::query_with(
            &EmployeeExpenses::build_insert_string(EmployeeExpenses::TABLE, &EmployeeExpenses::COLUMNS_ARRAY),
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

    async fn update(connection: PgPool, entity: &EmployeeExpenses) -> Result<bool, Box<dyn StdError>> {
        sqlx::query_with(
            &EmployeeExpenses::build_update_string(EmployeeExpenses::TABLE, &EmployeeExpenses::COLUMNS_ARRAY, EmployeeExpenses::PK),
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
            &EmployeeExpenses::build_delete_string(EmployeeExpenses::TABLE, EmployeeExpenses::PK),
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
