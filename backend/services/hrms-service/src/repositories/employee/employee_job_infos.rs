use crate::models::employee::employee_job_infos::EmployeeJobInfos;
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

pub struct EmployeeJobInfosRepository {}

impl IRepository<EmployeeJobInfos> for EmployeeJobInfosRepository {
    async fn get_all(connection: PgPool) -> Result<Vec<EmployeeJobInfos>, Box<dyn StdError>> {
        let result = sqlx::query(format!("SELECT {} FROM {}", EmployeeJobInfos::COLUMNS, EmployeeJobInfos::TABLE).as_str())
            .map(|row: PgRow| EmployeeJobInfos::from_row(&row))
            .fetch_all(&connection)
            .await
            .map_err(|e| Box::new(e) as Box<dyn StdError>)?;

        Ok(result)
    }

    async fn get_by_filter(
        connection: PgPool,
        filter: &String,
    ) -> Result<Vec<EmployeeJobInfos>, Box<dyn StdError>> {
        let query = format!(
            r#"SELECT {} FROM {} WHERE {}"#,
            EmployeeJobInfos::COLUMNS,
            EmployeeJobInfos::TABLE,
            filter
        );
        let result = sqlx::query(query.as_str())
            .map(|row: PgRow| EmployeeJobInfos::from_row(&row))
            .fetch_all(&connection)
            .await
            .map_err(|e| Box::new(e) as Box<dyn StdError>)?;

        Ok(result)
    }

    async fn add(connection: PgPool, entity: &EmployeeJobInfos) -> Result<bool, Box<dyn StdError>> {
        let mut args = PgArguments::default();
        let _ = args.add(entity.employee_job_info_id.clone());
let _ = args.add(entity.employee_id.clone());
let _ = args.add(entity.department_id.clone());
let _ = args.add(entity.designation_id.clone());
let _ = args.add(entity.grade_id.clone());
let _ = args.add(entity.employee_type_id.clone());
let _ = args.add(entity.job_status_id.clone());
let _ = args.add(entity.date_of_joining.clone());
let _ = args.add(entity.employee_code.clone());
let _ = args.add(entity.status_id.clone());
let _ = args.add(entity.end_date.clone());

        sqlx::query_with(
            format!(
                "INSERT INTO {} ({}) VALUES ($1, $2, $3, $4)",
                EmployeeJobInfos::TABLE,
                EmployeeJobInfos::COLUMNS
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

    async fn update(connection: PgPool, entity: &EmployeeJobInfos) -> Result<bool, Box<dyn StdError>> {
        let mut args = PgArguments::default();
        let _ = args.add(entity.employee_job_info_id.clone());
let _ = args.add(entity.employee_id.clone());
let _ = args.add(entity.department_id.clone());
let _ = args.add(entity.designation_id.clone());
let _ = args.add(entity.grade_id.clone());
let _ = args.add(entity.employee_type_id.clone());
let _ = args.add(entity.job_status_id.clone());
let _ = args.add(entity.date_of_joining.clone());
let _ = args.add(entity.employee_code.clone());
let _ = args.add(entity.status_id.clone());
let _ = args.add(entity.end_date.clone());

        sqlx::query_with(
            format!("UPDATE {} SET {}", EmployeeJobInfos::TABLE, EmployeeJobInfos::COLUMNS_UPDATE).as_str(),
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
            format!("DELETE FROM {} WHERE {}", EmployeeJobInfos::TABLE, EmployeeJobInfos::PK).as_str(),
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
