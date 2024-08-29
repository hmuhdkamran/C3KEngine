use crate::{
    models::employee::employee_educations::EmployeeEducations,
    repositories::employee::employee_educations::EmployeeEducationsRepository,
};
use c3k_common::{
    interfaces::{irepository::IRepository, iservice::IService},
    models::response::ApiResponse,
};
pub use sqlx::PgPool;

pub struct EmployeeEducationsService {}

impl IService<EmployeeEducations> for EmployeeEducationsService {
    async fn get_all(connection: PgPool) -> ApiResponse<Vec<EmployeeEducations>> {
        match EmployeeEducationsRepository::get_all(connection).await {
            Ok(entity) => ApiResponse::success(entity),
            Err(e) => ApiResponse::error(e.to_string()),
        }
    }

    async fn get_by_filter(connection: PgPool, filter: &String) -> ApiResponse<Vec<EmployeeEducations>> {
        match EmployeeEducationsRepository::get_by_filter(connection, filter).await {
            Ok(entity) => ApiResponse::success(entity),
            Err(e) => ApiResponse::error(e.to_string()),
        }
    }

    async fn add(connection: PgPool, entity: &EmployeeEducations) -> ApiResponse<bool> {
        match EmployeeEducationsRepository::add(connection, entity).await {
            Ok(entity) => ApiResponse::success(entity),
            Err(e) => ApiResponse::error(e.to_string()),
        }
    }

    async fn update(connection: PgPool, entity: &EmployeeEducations) -> ApiResponse<bool> {
        match EmployeeEducationsRepository::update(connection, entity).await {
            Ok(entity) => ApiResponse::success(entity),
            Err(e) => ApiResponse::error(e.to_string()),
        }
    }

    async fn delete(connection: PgPool, id: &String) -> ApiResponse<bool> {
        match EmployeeEducationsRepository::delete(connection, id).await {
            Ok(entity) => ApiResponse::success(entity),
            Err(e) => ApiResponse::error(e.to_string()),
        }
    }
}
