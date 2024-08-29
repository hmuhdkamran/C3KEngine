use crate::{
    models::employee::employee_leave_entitlements::EmployeeLeaveEntitlements,
    repositories::employee::employee_leave_entitlements::EmployeeLeaveEntitlementsRepository,
};
use c3k_common::{
    interfaces::{irepository::IRepository, iservice::IService},
    models::response::ApiResponse,
};
pub use sqlx::PgPool;

pub struct EmployeeLeaveEntitlementsService {}

impl IService<EmployeeLeaveEntitlements> for EmployeeLeaveEntitlementsService {
    async fn get_all(connection: PgPool) -> ApiResponse<Vec<EmployeeLeaveEntitlements>> {
        match EmployeeLeaveEntitlementsRepository::get_all(connection).await {
            Ok(entity) => ApiResponse::success(entity),
            Err(e) => ApiResponse::error(e.to_string()),
        }
    }

    async fn get_by_filter(connection: PgPool, filter: &String) -> ApiResponse<Vec<EmployeeLeaveEntitlements>> {
        match EmployeeLeaveEntitlementsRepository::get_by_filter(connection, filter).await {
            Ok(entity) => ApiResponse::success(entity),
            Err(e) => ApiResponse::error(e.to_string()),
        }
    }

    async fn add(connection: PgPool, entity: &EmployeeLeaveEntitlements) -> ApiResponse<bool> {
        match EmployeeLeaveEntitlementsRepository::add(connection, entity).await {
            Ok(entity) => ApiResponse::success(entity),
            Err(e) => ApiResponse::error(e.to_string()),
        }
    }

    async fn update(connection: PgPool, entity: &EmployeeLeaveEntitlements) -> ApiResponse<bool> {
        match EmployeeLeaveEntitlementsRepository::update(connection, entity).await {
            Ok(entity) => ApiResponse::success(entity),
            Err(e) => ApiResponse::error(e.to_string()),
        }
    }

    async fn delete(connection: PgPool, id: &String) -> ApiResponse<bool> {
        match EmployeeLeaveEntitlementsRepository::delete(connection, id).await {
            Ok(entity) => ApiResponse::success(entity),
            Err(e) => ApiResponse::error(e.to_string()),
        }
    }
}
