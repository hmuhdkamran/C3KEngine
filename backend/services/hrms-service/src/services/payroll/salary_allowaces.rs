use crate::{
    models::payroll::salary_allowaces::SalaryAllowaces,
    repositories::payroll::salary_allowaces::SalaryAllowacesRepository,
};
use c3k_common::{
    interfaces::{irepository::IRepository, iservice::IService},
    models::response::ApiResponse,
};
pub use sqlx::PgPool;

pub struct SalaryAllowacesService {}

impl IService<SalaryAllowaces> for SalaryAllowacesService {
    async fn get_all(connection: PgPool) -> ApiResponse<Vec<SalaryAllowaces>> {
        match SalaryAllowacesRepository::get_all(connection).await {
            Ok(entity) => ApiResponse::success(entity),
            Err(e) => ApiResponse::error(e.to_string()),
        }
    }

    async fn get_by_filter(connection: PgPool, filter: &String) -> ApiResponse<Vec<SalaryAllowaces>> {
        match SalaryAllowacesRepository::get_by_filter(connection, filter).await {
            Ok(entity) => ApiResponse::success(entity),
            Err(e) => ApiResponse::error(e.to_string()),
        }
    }

    async fn add(connection: PgPool, entity: &SalaryAllowaces) -> ApiResponse<bool> {
        match SalaryAllowacesRepository::add(connection, entity).await {
            Ok(entity) => ApiResponse::success(entity),
            Err(e) => ApiResponse::error(e.to_string()),
        }
    }

    async fn update(connection: PgPool, entity: &SalaryAllowaces) -> ApiResponse<bool> {
        match SalaryAllowacesRepository::update(connection, entity).await {
            Ok(entity) => ApiResponse::success(entity),
            Err(e) => ApiResponse::error(e.to_string()),
        }
    }

    async fn delete(connection: PgPool, id: &String) -> ApiResponse<bool> {
        match SalaryAllowacesRepository::delete(connection, id).await {
            Ok(entity) => ApiResponse::success(entity),
            Err(e) => ApiResponse::error(e.to_string()),
        }
    }
}
