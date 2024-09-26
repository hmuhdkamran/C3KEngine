use crate::models::point_of_sale::sale_order_payments::SaleOrderPayments;
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

pub struct SaleOrderPaymentsRepository {}

impl IRepository<SaleOrderPayments> for SaleOrderPaymentsRepository {
    async fn get_all(connection: PgPool) -> Result<Vec<SaleOrderPayments>, Box<dyn StdError>> {
        let result = sqlx::query(format!("SELECT {} FROM {}", SaleOrderPayments::COLUMNS, SaleOrderPayments::TABLE).as_str())
            .map(|row: PgRow| SaleOrderPayments::from_row(&row))
            .fetch_all(&connection)
            .await
            .map_err(|e| Box::new(e) as Box<dyn StdError>)?;

        Ok(result)
    }

    async fn get_by_filter(
        connection: PgPool,
        filter: &String,
    ) -> Result<Vec<SaleOrderPayments>, Box<dyn StdError>> {
        let query = format!(
            r#"SELECT {} FROM {} WHERE {}"#,
            SaleOrderPayments::COLUMNS,
            SaleOrderPayments::TABLE,
            filter
        );
        let result = sqlx::query(query.as_str())
            .map(|row: PgRow| SaleOrderPayments::from_row(&row))
            .fetch_all(&connection)
            .await
            .map_err(|e| Box::new(e) as Box<dyn StdError>)?;

        Ok(result)
    }

    async fn add(connection: PgPool, entity: &SaleOrderPayments) -> Result<bool, Box<dyn StdError>> {
        let mut args = PgArguments::default();
        let _ = args.add(entity.sale_order_payment_id.clone());
let _ = args.add(entity.payment_type_id.clone());
let _ = args.add(entity.sale_order_id.clone());
let _ = args.add(entity.net_amount.clone());
let _ = args.add(entity.pay_amount.clone());
let _ = args.add(entity.balance.clone());
let _ = args.add(entity.status_id.clone());
let _ = args.add(entity.payment_term_id.clone());

        sqlx::query_with(
            format!(
                "INSERT INTO {} ({}) VALUES ($1, $2, $3, $4)",
                SaleOrderPayments::TABLE,
                SaleOrderPayments::COLUMNS
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

    async fn update(connection: PgPool, entity: &SaleOrderPayments) -> Result<bool, Box<dyn StdError>> {
        let mut args = PgArguments::default();
        let _ = args.add(entity.sale_order_payment_id.clone());
let _ = args.add(entity.payment_type_id.clone());
let _ = args.add(entity.sale_order_id.clone());
let _ = args.add(entity.net_amount.clone());
let _ = args.add(entity.pay_amount.clone());
let _ = args.add(entity.balance.clone());
let _ = args.add(entity.status_id.clone());
let _ = args.add(entity.payment_term_id.clone());

        sqlx::query_with(
            format!("UPDATE {} SET {}", SaleOrderPayments::TABLE, SaleOrderPayments::COLUMNS_UPDATE).as_str(),
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
            format!("DELETE FROM {} WHERE {}", SaleOrderPayments::TABLE, SaleOrderPayments::PK).as_str(),
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
