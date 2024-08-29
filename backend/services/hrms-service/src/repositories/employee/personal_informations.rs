use crate::models::employee::personal_informations::PersonalInformations;
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

pub struct PersonalInformationsRepository {}

impl IRepository<PersonalInformations> for PersonalInformationsRepository {
    async fn get_all(connection: PgPool) -> Result<Vec<PersonalInformations>, Box<dyn StdError>> {
        let result = sqlx::query(format!("SELECT {} FROM {}", PersonalInformations::COLUMNS, PersonalInformations::TABLE).as_str())
            .map(|row: PgRow| PersonalInformations::from_row(&row))
            .fetch_all(&connection)
            .await
            .map_err(|e| Box::new(e) as Box<dyn StdError>)?;

        Ok(result)
    }

    async fn get_by_filter(
        connection: PgPool,
        filter: &String,
    ) -> Result<Vec<PersonalInformations>, Box<dyn StdError>> {
        let query = format!(
            r#"SELECT {} FROM {} WHERE {}"#,
            PersonalInformations::COLUMNS,
            PersonalInformations::TABLE,
            filter
        );
        let result = sqlx::query(query.as_str())
            .map(|row: PgRow| PersonalInformations::from_row(&row))
            .fetch_all(&connection)
            .await
            .map_err(|e| Box::new(e) as Box<dyn StdError>)?;

        Ok(result)
    }

    async fn add(connection: PgPool, entity: &PersonalInformations) -> Result<bool, Box<dyn StdError>> {
        let mut args = PgArguments::default();
        let _ = args.add(entity.employee_id.clone());
let _ = args.add(entity.first_name.clone());
let _ = args.add(entity.middle_name.clone());
let _ = args.add(entity.last_name.clone());
let _ = args.add(entity.employee_code.clone());
let _ = args.add(entity.picture.clone());
let _ = args.add(entity.blood_group_id.clone());
let _ = args.add(entity.attendance_machine_no.clone());
let _ = args.add(entity.gender.clone());
let _ = args.add(entity.date_of_birth.clone());
let _ = args.add(entity.nationality_id.clone());
let _ = args.add(entity.religion_id.clone());
let _ = args.add(entity.martial_status.clone());

        sqlx::query_with(
            format!(
                "INSERT INTO {} ({}) VALUES ($1, $2, $3, $4)",
                PersonalInformations::TABLE,
                PersonalInformations::COLUMNS
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

    async fn update(connection: PgPool, entity: &PersonalInformations) -> Result<bool, Box<dyn StdError>> {
        let mut args = PgArguments::default();
        let _ = args.add(entity.employee_id.clone());
let _ = args.add(entity.first_name.clone());
let _ = args.add(entity.middle_name.clone());
let _ = args.add(entity.last_name.clone());
let _ = args.add(entity.employee_code.clone());
let _ = args.add(entity.picture.clone());
let _ = args.add(entity.blood_group_id.clone());
let _ = args.add(entity.attendance_machine_no.clone());
let _ = args.add(entity.gender.clone());
let _ = args.add(entity.date_of_birth.clone());
let _ = args.add(entity.nationality_id.clone());
let _ = args.add(entity.religion_id.clone());
let _ = args.add(entity.martial_status.clone());

        sqlx::query_with(
            format!("UPDATE {} SET {}", PersonalInformations::TABLE, PersonalInformations::COLUMNS_UPDATE).as_str(),
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
            format!("DELETE FROM {} WHERE {}", PersonalInformations::TABLE, PersonalInformations::PK).as_str(),
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
