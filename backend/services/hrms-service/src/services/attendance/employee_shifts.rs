use crate::{
    models::attendance::employee_shifts::EmployeeShifts,
    repositories::attendance::employee_shifts::EmployeeShiftsRepository,
};
use c3k_common::{
    interfaces::{irepository::IRepository, iservice::IService},
    models::response::ApiResponse,
};
pub use sqlx::PgPool;

pub struct EmployeeShiftsService {}

impl IService<EmployeeShifts> for EmployeeShiftsService {
    async fn get_all(connection: PgPool) -> ApiResponse<Vec<EmployeeShifts>> {
        match EmployeeShiftsRepository::get_all(connection).await {
            Ok(entity) => ApiResponse::success(entity),
            Err(e) => ApiResponse::error(e.to_string()),
        }
    }

    async fn get_by_filter(connection: PgPool, filter: &String) -> ApiResponse<Vec<EmployeeShifts>> {
        match EmployeeShiftsRepository::get_by_filter(connection, filter).await {
            Ok(entity) => ApiResponse::success(entity),
            Err(e) => ApiResponse::error(e.to_string()),
        }
    }

    async fn add(connection: PgPool, entity: &EmployeeShifts) -> ApiResponse<bool> {
        match EmployeeShiftsRepository::add(connection, entity).await {
            Ok(entity) => ApiResponse::success(entity),
            Err(e) => ApiResponse::error(e.to_string()),
        }
    }

    async fn update(connection: PgPool, entity: &EmployeeShifts) -> ApiResponse<bool> {
        match EmployeeShiftsRepository::update(connection, entity).await {
            Ok(entity) => ApiResponse::success(entity),
            Err(e) => ApiResponse::error(e.to_string()),
        }
    }

    async fn delete(connection: PgPool, id: &String) -> ApiResponse<bool> {
        match EmployeeShiftsRepository::delete(connection, id).await {
            Ok(entity) => ApiResponse::success(entity),
            Err(e) => ApiResponse::error(e.to_string()),
        }
    }
}
