use crate::models::setup::cities::Cities;
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

pub struct CitiesRepository {}

impl IRepository<Cities> for CitiesRepository {
    async fn get_all(connection: PgPool) -> Result<Vec<Cities>, Box<dyn StdError>> {
        let result = sqlx::query(format!("SELECT {} FROM {}", Cities::COLUMNS, Cities::TABLE).as_str())
            .map(|row: PgRow| Cities::from_row(&row))
            .fetch_all(&connection)
            .await
            .map_err(|e| Box::new(e) as Box<dyn StdError>)?;

        Ok(result)
    }

    async fn get_by_filter(
        connection: PgPool,
        filter: &String,
    ) -> Result<Vec<Cities>, Box<dyn StdError>> {
        let query = format!(
            r#"SELECT {} FROM {} WHERE {}"#,
            Cities::COLUMNS,
            Cities::TABLE,
            filter
        );
        let result = sqlx::query(query.as_str())
            .map(|row: PgRow| Cities::from_row(&row))
            .fetch_all(&connection)
            .await
            .map_err(|e| Box::new(e) as Box<dyn StdError>)?;

        Ok(result)
    }

    async fn add(connection: PgPool, entity: &Cities) -> Result<bool, Box<dyn StdError>> {
        let mut args = PgArguments::default();
        let _ = args.add(entity.city_id.clone());
let _ = args.add(entity.abbreviation.clone());
let _ = args.add(entity.full_name.clone());
let _ = args.add(entity.state_id.clone());
let _ = args.add(entity.status_id.clone());

        sqlx::query_with(
            format!(
                "INSERT INTO {} ({}) VALUES ($1, $2, $3, $4)",
                Cities::TABLE,
                Cities::COLUMNS
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

    async fn update(connection: PgPool, entity: &Cities) -> Result<bool, Box<dyn StdError>> {
        let mut args = PgArguments::default();
        let _ = args.add(entity.city_id.clone());
let _ = args.add(entity.abbreviation.clone());
let _ = args.add(entity.full_name.clone());
let _ = args.add(entity.state_id.clone());
let _ = args.add(entity.status_id.clone());

        sqlx::query_with(
            format!("UPDATE {} SET {}", Cities::TABLE, Cities::COLUMNS_UPDATE).as_str(),
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
            format!("DELETE FROM {} WHERE {}", Cities::TABLE, Cities::PK).as_str(),
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
