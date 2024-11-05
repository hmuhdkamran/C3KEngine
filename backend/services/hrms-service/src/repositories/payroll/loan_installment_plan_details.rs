use crate::models::payroll::loan_installment_plan_details::LoanInstallmentPlanDetails;
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

pub struct LoanInstallmentPlanDetailsRepository {}

impl IRepository<LoanInstallmentPlanDetails> for LoanInstallmentPlanDetailsRepository {
    async fn get_all(connection: PgPool) -> Result<Vec<LoanInstallmentPlanDetails>, Box<dyn StdError>> {
        let result = sqlx::query(&LoanInstallmentPlanDetails::build_select_string(
            LoanInstallmentPlanDetails::TABLE,
            &LoanInstallmentPlanDetails::COLUMNS_ARRAY,
            None,
        ))
        .map(|row: PgRow| LoanInstallmentPlanDetails::from_row(&row))
        .fetch_all(&connection)
        .await
        .map_err(|e| Box::new(e) as Box<dyn StdError>)?;

        Ok(result)
    }

    async fn get_by_filter(
        connection: PgPool,
        filter: &String,
    ) -> Result<Vec<LoanInstallmentPlanDetails>, Box<dyn StdError>> {
        let result = sqlx::query(&LoanInstallmentPlanDetails::build_select_string(
            LoanInstallmentPlanDetails::TABLE,
            &LoanInstallmentPlanDetails::COLUMNS_ARRAY,
            Some(filter),
        ))
        .map(|row: PgRow| LoanInstallmentPlanDetails::from_row(&row))
        .fetch_all(&connection)
        .await
        .map_err(|e| Box::new(e) as Box<dyn StdError>)?;

        Ok(result)
    }

    async fn add(connection: PgPool, entity: &LoanInstallmentPlanDetails) -> Result<bool, Box<dyn StdError>> {
        sqlx::query_with(
            &LoanInstallmentPlanDetails::build_insert_string(LoanInstallmentPlanDetails::TABLE, &LoanInstallmentPlanDetails::COLUMNS_ARRAY),
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

    async fn update(connection: PgPool, entity: &LoanInstallmentPlanDetails) -> Result<bool, Box<dyn StdError>> {
        sqlx::query_with(
            &LoanInstallmentPlanDetails::build_update_string(LoanInstallmentPlanDetails::TABLE, &LoanInstallmentPlanDetails::COLUMNS_ARRAY, LoanInstallmentPlanDetails::PK),
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
            &LoanInstallmentPlanDetails::build_delete_string(LoanInstallmentPlanDetails::TABLE, LoanInstallmentPlanDetails::PK),
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
