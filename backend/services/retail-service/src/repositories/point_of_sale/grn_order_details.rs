use crate::models::point_of_sale::grn_order_details::GrnOrderDetails;
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

pub struct GrnOrderDetailsRepository {}

impl IRepository<GrnOrderDetails> for GrnOrderDetailsRepository {
    async fn get_all(connection: PgPool) -> Result<Vec<GrnOrderDetails>, Box<dyn StdError>> {
        let result = sqlx::query(format!("SELECT {} FROM {}", GrnOrderDetails::COLUMNS, GrnOrderDetails::TABLE).as_str())
            .map(|row: PgRow| GrnOrderDetails::from_row(&row))
            .fetch_all(&connection)
            .await
            .map_err(|e| Box::new(e) as Box<dyn StdError>)?;

        Ok(result)
    }

    async fn get_by_filter(
        connection: PgPool,
        filter: &String,
    ) -> Result<Vec<GrnOrderDetails>, Box<dyn StdError>> {
        let query = format!(
            r#"SELECT {} FROM {} WHERE {}"#,
            GrnOrderDetails::COLUMNS,
            GrnOrderDetails::TABLE,
            filter
        );
        let result = sqlx::query(query.as_str())
            .map(|row: PgRow| GrnOrderDetails::from_row(&row))
            .fetch_all(&connection)
            .await
            .map_err(|e| Box::new(e) as Box<dyn StdError>)?;

        Ok(result)
    }

    async fn add(connection: PgPool, entity: &GrnOrderDetails) -> Result<bool, Box<dyn StdError>> {
        let mut args = PgArguments::default();
        let _ = args.add(entity.grn_order_detail_id.clone());
let _ = args.add(entity.grn_order_id.clone());
let _ = args.add(entity.product_id.clone());
let _ = args.add(entity.qunatity.clone());
let _ = args.add(entity.warehouse_id.clone());
let _ = args.add(entity.sale_price.clone());
let _ = args.add(entity.purchase_price.clone());
let _ = args.add(entity.whole_sale_price.clone());
let _ = args.add(entity.net_price.clone());
let _ = args.add(entity.status_id.clone());

        sqlx::query_with(
            format!(
                "INSERT INTO {} ({}) VALUES ($1, $2, $3, $4)",
                GrnOrderDetails::TABLE,
                GrnOrderDetails::COLUMNS
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

    async fn update(connection: PgPool, entity: &GrnOrderDetails) -> Result<bool, Box<dyn StdError>> {
        let mut args = PgArguments::default();
        let _ = args.add(entity.grn_order_detail_id.clone());
let _ = args.add(entity.grn_order_id.clone());
let _ = args.add(entity.product_id.clone());
let _ = args.add(entity.qunatity.clone());
let _ = args.add(entity.warehouse_id.clone());
let _ = args.add(entity.sale_price.clone());
let _ = args.add(entity.purchase_price.clone());
let _ = args.add(entity.whole_sale_price.clone());
let _ = args.add(entity.net_price.clone());
let _ = args.add(entity.status_id.clone());

        sqlx::query_with(
            format!("UPDATE {} SET {}", GrnOrderDetails::TABLE, GrnOrderDetails::COLUMNS_UPDATE).as_str(),
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
            format!("DELETE FROM {} WHERE {}", GrnOrderDetails::TABLE, GrnOrderDetails::PK).as_str(),
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
