use crate::models::employee::employee_contacts::EmployeeContacts;
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

pub struct EmployeeContactsRepository {}

impl IRepository<EmployeeContacts> for EmployeeContactsRepository {
    async fn get_all(connection: PgPool) -> Result<Vec<EmployeeContacts>, Box<dyn StdError>> {
        let result = sqlx::query(&EmployeeContacts::build_select_string(
            EmployeeContacts::TABLE,
            &EmployeeContacts::COLUMNS_ARRAY,
            None,
        ))
        .map(|row: PgRow| EmployeeContacts::from_row(&row))
        .fetch_all(&connection)
        .await
        .map_err(|e| Box::new(e) as Box<dyn StdError>)?;

        Ok(result)
    }

    async fn get_by_filter(
        connection: PgPool,
        filter: &String,
    ) -> Result<Vec<EmployeeContacts>, Box<dyn StdError>> {
        let result = sqlx::query(&EmployeeContacts::build_select_string(
            EmployeeContacts::TABLE,
            &EmployeeContacts::COLUMNS_ARRAY,
            Some(filter),
        ))
        .map(|row: PgRow| EmployeeContacts::from_row(&row))
        .fetch_all(&connection)
        .await
        .map_err(|e| Box::new(e) as Box<dyn StdError>)?;

        Ok(result)
    }

    async fn add(connection: PgPool, entity: &EmployeeContacts) -> Result<bool, Box<dyn StdError>> {
        sqlx::query_with(
            &EmployeeContacts::build_insert_string(EmployeeContacts::TABLE, &EmployeeContacts::COLUMNS_ARRAY),
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

    async fn update(connection: PgPool, entity: &EmployeeContacts) -> Result<bool, Box<dyn StdError>> {
        sqlx::query_with(
            &EmployeeContacts::build_update_string(EmployeeContacts::TABLE, &EmployeeContacts::COLUMNS_ARRAY, EmployeeContacts::PK),
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
            &EmployeeContacts::build_delete_string(EmployeeContacts::TABLE, EmployeeContacts::PK),
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
