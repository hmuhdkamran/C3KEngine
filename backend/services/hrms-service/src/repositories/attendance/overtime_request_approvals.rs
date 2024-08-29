use crate::models::attendance::overtime_request_approvals::OvertimeRequestApprovals;
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

pub struct OvertimeRequestApprovalsRepository {}

impl IRepository<OvertimeRequestApprovals> for OvertimeRequestApprovalsRepository {
    async fn get_all(connection: PgPool) -> Result<Vec<OvertimeRequestApprovals>, Box<dyn StdError>> {
        let result = sqlx::query(format!("SELECT {} FROM {}", OvertimeRequestApprovals::COLUMNS, OvertimeRequestApprovals::TABLE).as_str())
            .map(|row: PgRow| OvertimeRequestApprovals::from_row(&row))
            .fetch_all(&connection)
            .await
            .map_err(|e| Box::new(e) as Box<dyn StdError>)?;

        Ok(result)
    }

    async fn get_by_filter(
        connection: PgPool,
        filter: &String,
    ) -> Result<Vec<OvertimeRequestApprovals>, Box<dyn StdError>> {
        let query = format!(
            r#"SELECT {} FROM {} WHERE {}"#,
            OvertimeRequestApprovals::COLUMNS,
            OvertimeRequestApprovals::TABLE,
            filter
        );
        let result = sqlx::query(query.as_str())
            .map(|row: PgRow| OvertimeRequestApprovals::from_row(&row))
            .fetch_all(&connection)
            .await
            .map_err(|e| Box::new(e) as Box<dyn StdError>)?;

        Ok(result)
    }

    async fn add(connection: PgPool, entity: &OvertimeRequestApprovals) -> Result<bool, Box<dyn StdError>> {
        let mut args = PgArguments::default();
        let _ = args.add(entity.overtime_request_approval_id.clone());
let _ = args.add(entity.overtime_request_id.clone());
let _ = args.add(entity.approved_by.clone());
let _ = args.add(entity.approved_date.clone());
let _ = args.add(entity.application_status_id.clone());
let _ = args.add(entity.comments.clone());
let _ = args.add(entity.status_id.clone());

        sqlx::query_with(
            format!(
                "INSERT INTO {} ({}) VALUES ($1, $2, $3, $4)",
                OvertimeRequestApprovals::TABLE,
                OvertimeRequestApprovals::COLUMNS
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

    async fn update(connection: PgPool, entity: &OvertimeRequestApprovals) -> Result<bool, Box<dyn StdError>> {
        let mut args = PgArguments::default();
        let _ = args.add(entity.overtime_request_approval_id.clone());
let _ = args.add(entity.overtime_request_id.clone());
let _ = args.add(entity.approved_by.clone());
let _ = args.add(entity.approved_date.clone());
let _ = args.add(entity.application_status_id.clone());
let _ = args.add(entity.comments.clone());
let _ = args.add(entity.status_id.clone());

        sqlx::query_with(
            format!("UPDATE {} SET {}", OvertimeRequestApprovals::TABLE, OvertimeRequestApprovals::COLUMNS_UPDATE).as_str(),
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
            format!("DELETE FROM {} WHERE {}", OvertimeRequestApprovals::TABLE, OvertimeRequestApprovals::PK).as_str(),
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
