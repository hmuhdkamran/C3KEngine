use crate::models::point_of_sale::grn_orders::GrnOrders;
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

pub struct GrnOrdersRepository {}

impl IRepository<GrnOrders> for GrnOrdersRepository {
    async fn get_all(connection: PgPool) -> Result<Vec<GrnOrders>, Box<dyn StdError>> {
        let result = sqlx::query(&GrnOrders::build_select_string(
            GrnOrders::TABLE,
            &GrnOrders::COLUMNS_ARRAY,
            None,
        ))
        .map(|row: PgRow| GrnOrders::from_row(&row))
        .fetch_all(&connection)
        .await
        .map_err(|e| Box::new(e) as Box<dyn StdError>)?;

        Ok(result)
    }

    async fn get_by_filter(
        connection: PgPool,
        filter: &String,
    ) -> Result<Vec<GrnOrders>, Box<dyn StdError>> {
        let result = sqlx::query(&GrnOrders::build_select_string(
            GrnOrders::TABLE,
            &GrnOrders::COLUMNS_ARRAY,
            Some(filter),
        ))
        .map(|row: PgRow| GrnOrders::from_row(&row))
        .fetch_all(&connection)
        .await
        .map_err(|e| Box::new(e) as Box<dyn StdError>)?;

        Ok(result)
    }

    async fn add(connection: PgPool, entity: &GrnOrders) -> Result<bool, Box<dyn StdError>> {
        sqlx::query_with(
            &GrnOrders::build_insert_string(GrnOrders::TABLE, &GrnOrders::COLUMNS_ARRAY),
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

    async fn update(connection: PgPool, entity: &GrnOrders) -> Result<bool, Box<dyn StdError>> {
        sqlx::query_with(
            &GrnOrders::build_update_string(GrnOrders::TABLE, &GrnOrders::COLUMNS_ARRAY, GrnOrders::PK),
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
            &GrnOrders::build_delete_string(GrnOrders::TABLE, GrnOrders::PK),
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
