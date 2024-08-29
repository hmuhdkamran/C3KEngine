use crate::models::recruitment::job_posts::JobPosts;
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

pub struct JobPostsRepository {}

impl IRepository<JobPosts> for JobPostsRepository {
    async fn get_all(connection: PgPool) -> Result<Vec<JobPosts>, Box<dyn StdError>> {
        let result = sqlx::query(format!("SELECT {} FROM {}", JobPosts::COLUMNS, JobPosts::TABLE).as_str())
            .map(|row: PgRow| JobPosts::from_row(&row))
            .fetch_all(&connection)
            .await
            .map_err(|e| Box::new(e) as Box<dyn StdError>)?;

        Ok(result)
    }

    async fn get_by_filter(
        connection: PgPool,
        filter: &String,
    ) -> Result<Vec<JobPosts>, Box<dyn StdError>> {
        let query = format!(
            r#"SELECT {} FROM {} WHERE {}"#,
            JobPosts::COLUMNS,
            JobPosts::TABLE,
            filter
        );
        let result = sqlx::query(query.as_str())
            .map(|row: PgRow| JobPosts::from_row(&row))
            .fetch_all(&connection)
            .await
            .map_err(|e| Box::new(e) as Box<dyn StdError>)?;

        Ok(result)
    }

    async fn add(connection: PgPool, entity: &JobPosts) -> Result<bool, Box<dyn StdError>> {
        let mut args = PgArguments::default();
        let _ = args.add(entity.job_post_id.clone());
let _ = args.add(entity.title.clone());
let _ = args.add(entity.short_description.clone());
let _ = args.add(entity.long_description.clone());
let _ = args.add(entity.gender.clone());
let _ = args.add(entity.age_limit.clone());
let _ = args.add(entity.total_position.clone());
let _ = args.add(entity.job_status_id.clone());
let _ = args.add(entity.posted_date.clone());
let _ = args.add(entity.expiry_dated.clone());
let _ = args.add(entity.minimum_education_id.clone());
let _ = args.add(entity.apply_before.clone());
let _ = args.add(entity.industry_id.clone());
let _ = args.add(entity.city_id.clone());
let _ = args.add(entity.job_shift_id.clone());
let _ = args.add(entity.career_level_id.clone());
let _ = args.add(entity.functional_area_id.clone());
let _ = args.add(entity.job_experience_id.clone());
let _ = args.add(entity.department_id.clone());
let _ = args.add(entity.salary_from.clone());
let _ = args.add(entity.salary_to.clone());

        sqlx::query_with(
            format!(
                "INSERT INTO {} ({}) VALUES ($1, $2, $3, $4)",
                JobPosts::TABLE,
                JobPosts::COLUMNS
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

    async fn update(connection: PgPool, entity: &JobPosts) -> Result<bool, Box<dyn StdError>> {
        let mut args = PgArguments::default();
        let _ = args.add(entity.job_post_id.clone());
let _ = args.add(entity.title.clone());
let _ = args.add(entity.short_description.clone());
let _ = args.add(entity.long_description.clone());
let _ = args.add(entity.gender.clone());
let _ = args.add(entity.age_limit.clone());
let _ = args.add(entity.total_position.clone());
let _ = args.add(entity.job_status_id.clone());
let _ = args.add(entity.posted_date.clone());
let _ = args.add(entity.expiry_dated.clone());
let _ = args.add(entity.minimum_education_id.clone());
let _ = args.add(entity.apply_before.clone());
let _ = args.add(entity.industry_id.clone());
let _ = args.add(entity.city_id.clone());
let _ = args.add(entity.job_shift_id.clone());
let _ = args.add(entity.career_level_id.clone());
let _ = args.add(entity.functional_area_id.clone());
let _ = args.add(entity.job_experience_id.clone());
let _ = args.add(entity.department_id.clone());
let _ = args.add(entity.salary_from.clone());
let _ = args.add(entity.salary_to.clone());

        sqlx::query_with(
            format!("UPDATE {} SET {}", JobPosts::TABLE, JobPosts::COLUMNS_UPDATE).as_str(),
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
            format!("DELETE FROM {} WHERE {}", JobPosts::TABLE, JobPosts::PK).as_str(),
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
