use crate::models::point_of_sale::campaign_products::CampaignProducts;
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

pub struct CampaignProductsRepository {}

impl IRepository<CampaignProducts> for CampaignProductsRepository {
    async fn get_all(connection: PgPool) -> Result<Vec<CampaignProducts>, Box<dyn StdError>> {
        let result = sqlx::query(&CampaignProducts::build_select_string(
            CampaignProducts::TABLE,
            &CampaignProducts::COLUMNS_ARRAY,
            None,
        ))
        .map(|row: PgRow| CampaignProducts::from_row(&row))
        .fetch_all(&connection)
        .await
        .map_err(|e| Box::new(e) as Box<dyn StdError>)?;

        Ok(result)
    }

    async fn get_by_filter(
        connection: PgPool,
        filter: &String,
    ) -> Result<Vec<CampaignProducts>, Box<dyn StdError>> {
        let result = sqlx::query(&CampaignProducts::build_select_string(
            CampaignProducts::TABLE,
            &CampaignProducts::COLUMNS_ARRAY,
            Some(filter),
        ))
        .map(|row: PgRow| CampaignProducts::from_row(&row))
        .fetch_all(&connection)
        .await
        .map_err(|e| Box::new(e) as Box<dyn StdError>)?;

        Ok(result)
    }

    async fn add(connection: PgPool, entity: &CampaignProducts) -> Result<bool, Box<dyn StdError>> {
        sqlx::query_with(
            &CampaignProducts::build_insert_string(CampaignProducts::TABLE, &CampaignProducts::COLUMNS_ARRAY),
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

    async fn update(connection: PgPool, entity: &CampaignProducts) -> Result<bool, Box<dyn StdError>> {
        sqlx::query_with(
            &CampaignProducts::build_update_string(CampaignProducts::TABLE, &CampaignProducts::COLUMNS_ARRAY, CampaignProducts::PK),
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
            &CampaignProducts::build_delete_string(CampaignProducts::TABLE, CampaignProducts::PK),
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
