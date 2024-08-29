use crate::models::payroll::salaries::Salaries;
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

pub struct SalariesRepository {}

impl IRepository<Salaries> for SalariesRepository {
    async fn get_all(connection: PgPool) -> Result<Vec<Salaries>, Box<dyn StdError>> {
        let result = sqlx::query(format!("SELECT {} FROM {}", Salaries::COLUMNS, Salaries::TABLE).as_str())
            .map(|row: PgRow| Salaries::from_row(&row))
            .fetch_all(&connection)
            .await
            .map_err(|e| Box::new(e) as Box<dyn StdError>)?;

        Ok(result)
    }

    async fn get_by_filter(
        connection: PgPool,
        filter: &String,
    ) -> Result<Vec<Salaries>, Box<dyn StdError>> {
        let query = format!(
            r#"SELECT {} FROM {} WHERE {}"#,
            Salaries::COLUMNS,
            Salaries::TABLE,
            filter
        );
        let result = sqlx::query(query.as_str())
            .map(|row: PgRow| Salaries::from_row(&row))
            .fetch_all(&connection)
            .await
            .map_err(|e| Box::new(e) as Box<dyn StdError>)?;

        Ok(result)
    }

    async fn add(connection: PgPool, entity: &Salaries) -> Result<bool, Box<dyn StdError>> {
        let mut args = PgArguments::default();
        let _ = args.add(entity.salarie_id.clone());
let _ = args.add(entity.salary_type_id.clone());
let _ = args.add(entity.employee_id.clone());
let _ = args.add(entity.employee_job_info_id.clone());
let _ = args.add(entity.allowance_amount.clone());
let _ = args.add(entity.deduction_amount.clone());
let _ = args.add(entity.net_amount.clone());
let _ = args.add(entity.year_id.clone());
let _ = args.add(entity.week_id.clone());
let _ = args.add(entity.fort_night_id.clone());
let _ = args.add(entity.freezed.clone());
let _ = args.add(entity.status_id.clone());
let _ = args.add(entity.month_id.clone());

        sqlx::query_with(
            format!(
                "INSERT INTO {} ({}) VALUES ($1, $2, $3, $4)",
                Salaries::TABLE,
                Salaries::COLUMNS
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

    async fn update(connection: PgPool, entity: &Salaries) -> Result<bool, Box<dyn StdError>> {
        let mut args = PgArguments::default();
        let _ = args.add(entity.salarie_id.clone());
let _ = args.add(entity.salary_type_id.clone());
let _ = args.add(entity.employee_id.clone());
let _ = args.add(entity.employee_job_info_id.clone());
let _ = args.add(entity.allowance_amount.clone());
let _ = args.add(entity.deduction_amount.clone());
let _ = args.add(entity.net_amount.clone());
let _ = args.add(entity.year_id.clone());
let _ = args.add(entity.week_id.clone());
let _ = args.add(entity.fort_night_id.clone());
let _ = args.add(entity.freezed.clone());
let _ = args.add(entity.status_id.clone());
let _ = args.add(entity.month_id.clone());

        sqlx::query_with(
            format!("UPDATE {} SET {}", Salaries::TABLE, Salaries::COLUMNS_UPDATE).as_str(),
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
            format!("DELETE FROM {} WHERE {}", Salaries::TABLE, Salaries::PK).as_str(),
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
