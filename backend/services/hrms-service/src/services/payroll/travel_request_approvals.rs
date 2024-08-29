use crate::{
    models::payroll::travel_request_approvals::TravelRequestApprovals,
    repositories::payroll::travel_request_approvals::TravelRequestApprovalsRepository,
};
use c3k_common::{
    interfaces::{irepository::IRepository, iservice::IService},
    models::response::ApiResponse,
};
pub use sqlx::PgPool;

pub struct TravelRequestApprovalsService {}

impl IService<TravelRequestApprovals> for TravelRequestApprovalsService {
    async fn get_all(connection: PgPool) -> ApiResponse<Vec<TravelRequestApprovals>> {
        match TravelRequestApprovalsRepository::get_all(connection).await {
            Ok(entity) => ApiResponse::success(entity),
            Err(e) => ApiResponse::error(e.to_string()),
        }
    }

    async fn get_by_filter(connection: PgPool, filter: &String) -> ApiResponse<Vec<TravelRequestApprovals>> {
        match TravelRequestApprovalsRepository::get_by_filter(connection, filter).await {
            Ok(entity) => ApiResponse::success(entity),
            Err(e) => ApiResponse::error(e.to_string()),
        }
    }

    async fn add(connection: PgPool, entity: &TravelRequestApprovals) -> ApiResponse<bool> {
        match TravelRequestApprovalsRepository::add(connection, entity).await {
            Ok(entity) => ApiResponse::success(entity),
            Err(e) => ApiResponse::error(e.to_string()),
        }
    }

    async fn update(connection: PgPool, entity: &TravelRequestApprovals) -> ApiResponse<bool> {
        match TravelRequestApprovalsRepository::update(connection, entity).await {
            Ok(entity) => ApiResponse::success(entity),
            Err(e) => ApiResponse::error(e.to_string()),
        }
    }

    async fn delete(connection: PgPool, id: &String) -> ApiResponse<bool> {
        match TravelRequestApprovalsRepository::delete(connection, id).await {
            Ok(entity) => ApiResponse::success(entity),
            Err(e) => ApiResponse::error(e.to_string()),
        }
    }
}
