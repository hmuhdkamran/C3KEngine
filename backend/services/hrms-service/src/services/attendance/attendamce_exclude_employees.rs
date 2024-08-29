use crate::{
    models::attendance::attendamce_exclude_employees::AttendamceExcludeEmployees,
    repositories::attendance::attendamce_exclude_employees::AttendamceExcludeEmployeesRepository,
};
use c3k_common::{
    interfaces::{irepository::IRepository, iservice::IService},
    models::response::ApiResponse,
};
pub use sqlx::PgPool;

pub struct AttendamceExcludeEmployeesService {}

impl IService<AttendamceExcludeEmployees> for AttendamceExcludeEmployeesService {
    async fn get_all(connection: PgPool) -> ApiResponse<Vec<AttendamceExcludeEmployees>> {
        match AttendamceExcludeEmployeesRepository::get_all(connection).await {
            Ok(entity) => ApiResponse::success(entity),
            Err(e) => ApiResponse::error(e.to_string()),
        }
    }

    async fn get_by_filter(connection: PgPool, filter: &String) -> ApiResponse<Vec<AttendamceExcludeEmployees>> {
        match AttendamceExcludeEmployeesRepository::get_by_filter(connection, filter).await {
            Ok(entity) => ApiResponse::success(entity),
            Err(e) => ApiResponse::error(e.to_string()),
        }
    }

    async fn add(connection: PgPool, entity: &AttendamceExcludeEmployees) -> ApiResponse<bool> {
        match AttendamceExcludeEmployeesRepository::add(connection, entity).await {
            Ok(entity) => ApiResponse::success(entity),
            Err(e) => ApiResponse::error(e.to_string()),
        }
    }

    async fn update(connection: PgPool, entity: &AttendamceExcludeEmployees) -> ApiResponse<bool> {
        match AttendamceExcludeEmployeesRepository::update(connection, entity).await {
            Ok(entity) => ApiResponse::success(entity),
            Err(e) => ApiResponse::error(e.to_string()),
        }
    }

    async fn delete(connection: PgPool, id: &String) -> ApiResponse<bool> {
        match AttendamceExcludeEmployeesRepository::delete(connection, id).await {
            Ok(entity) => ApiResponse::success(entity),
            Err(e) => ApiResponse::error(e.to_string()),
        }
    }
}
