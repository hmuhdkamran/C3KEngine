use crate::models::employee::employee_addresses::EmployeeAddresses;
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

pub struct EmployeeAddressesRepository {}

impl IRepository<EmployeeAddresses> for EmployeeAddressesRepository {
    async fn get_all(connection: PgPool) -> Result<Vec<EmployeeAddresses>, Box<dyn StdError>> {
        let result = sqlx::query(format!("SELECT {} FROM {}", EmployeeAddresses::COLUMNS, EmployeeAddresses::TABLE).as_str())
            .map(|row: PgRow| EmployeeAddresses::from_row(&row))
            .fetch_all(&connection)
            .await
            .map_err(|e| Box::new(e) as Box<dyn StdError>)?;

        Ok(result)
    }

    async fn get_by_filter(
        connection: PgPool,
        filter: &String,
    ) -> Result<Vec<EmployeeAddresses>, Box<dyn StdError>> {
        let query = format!(
            r#"SELECT {} FROM {} WHERE {}"#,
            EmployeeAddresses::COLUMNS,
            EmployeeAddresses::TABLE,
            filter
        );
        let result = sqlx::query(query.as_str())
            .map(|row: PgRow| EmployeeAddresses::from_row(&row))
            .fetch_all(&connection)
            .await
            .map_err(|e| Box::new(e) as Box<dyn StdError>)?;

        Ok(result)
    }

    async fn add(connection: PgPool, entity: &EmployeeAddresses) -> Result<bool, Box<dyn StdError>> {
        let mut args = PgArguments::default();
        let _ = args.add(entity.employee_address_id.clone());
let _ = args.add(entity.employee_id.clone());
let _ = args.add(entity.street_address.clone());
let _ = args.add(entity.city_id.clone());
let _ = args.add(entity.state_id.clone());
let _ = args.add(entity.country_id.clone());
let _ = args.add(entity.address_type_id.clone());
let _ = args.add(entity.is_primary.clone());
let _ = args.add(entity.status_id.clone());

        sqlx::query_with(
            format!(
                "INSERT INTO {} ({}) VALUES ($1, $2, $3, $4)",
                EmployeeAddresses::TABLE,
                EmployeeAddresses::COLUMNS
            )
            .as_str(),
            args,
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

    async fn update(connection: PgPool, entity: &EmployeeAddresses) -> Result<bool, Box<dyn StdError>> {
        let mut args = PgArguments::default();
        let _ = args.add(entity.employee_address_id.clone());
let _ = args.add(entity.employee_id.clone());
let _ = args.add(entity.street_address.clone());
let _ = args.add(entity.city_id.clone());
let _ = args.add(entity.state_id.clone());
let _ = args.add(entity.country_id.clone());
let _ = args.add(entity.address_type_id.clone());
let _ = args.add(entity.is_primary.clone());
let _ = args.add(entity.status_id.clone());

        sqlx::query_with(
            format!("UPDATE {} SET {}", EmployeeAddresses::TABLE, EmployeeAddresses::COLUMNS_UPDATE).as_str(),
            args,
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
            format!("DELETE FROM {} WHERE {}", EmployeeAddresses::TABLE, EmployeeAddresses::PK).as_str(),
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
