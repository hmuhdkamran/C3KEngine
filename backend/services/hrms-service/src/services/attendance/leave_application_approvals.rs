use crate::{
    models::attendance::leave_application_approvals::LeaveApplicationApprovals,
    repositories::attendance::leave_application_approvals::LeaveApplicationApprovalsRepository,
};
use c3k_common::{
    interfaces::{irepository::IRepository, iservice::IService},
    models::response::ApiResponse,
};
pub use sqlx::PgPool;

pub struct LeaveApplicationApprovalsService {}

impl IService<LeaveApplicationApprovals> for LeaveApplicationApprovalsService {
    async fn get_all(connection: PgPool) -> ApiResponse<Vec<LeaveApplicationApprovals>> {
        match LeaveApplicationApprovalsRepository::get_all(connection).await {
            Ok(entity) => ApiResponse::success(entity),
            Err(e) => ApiResponse::error(e.to_string()),
        }
    }

    async fn get_by_filter(connection: PgPool, filter: &String) -> ApiResponse<Vec<LeaveApplicationApprovals>> {
        match LeaveApplicationApprovalsRepository::get_by_filter(connection, filter).await {
            Ok(entity) => ApiResponse::success(entity),
            Err(e) => ApiResponse::error(e.to_string()),
        }
    }

    async fn add(connection: PgPool, entity: &LeaveApplicationApprovals) -> ApiResponse<bool> {
        match LeaveApplicationApprovalsRepository::add(connection, entity).await {
            Ok(entity) => ApiResponse::success(entity),
            Err(e) => ApiResponse::error(e.to_string()),
        }
    }

    async fn update(connection: PgPool, entity: &LeaveApplicationApprovals) -> ApiResponse<bool> {
        match LeaveApplicationApprovalsRepository::update(connection, entity).await {
            Ok(entity) => ApiResponse::success(entity),
            Err(e) => ApiResponse::error(e.to_string()),
        }
    }

    async fn delete(connection: PgPool, id: &String) -> ApiResponse<bool> {
        match LeaveApplicationApprovalsRepository::delete(connection, id).await {
            Ok(entity) => ApiResponse::success(entity),
            Err(e) => ApiResponse::error(e.to_string()),
        }
    }
}
