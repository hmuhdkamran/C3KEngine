use crate::{
    models::payroll::tax_adjustment_requests::TaxAdjustmentRequests,
    repositories::payroll::tax_adjustment_requests::TaxAdjustmentRequestsRepository,
};
use c3k_common::{
    interfaces::{irepository::IRepository, iservice::IService},
    models::response::ApiResponse,
};
pub use sqlx::PgPool;

pub struct TaxAdjustmentRequestsService {}

impl IService<TaxAdjustmentRequests> for TaxAdjustmentRequestsService {
    async fn get_all(connection: PgPool) -> ApiResponse<Vec<TaxAdjustmentRequests>> {
        match TaxAdjustmentRequestsRepository::get_all(connection).await {
            Ok(entity) => ApiResponse::success(entity),
            Err(e) => ApiResponse::error(e.to_string()),
        }
    }

    async fn get_by_filter(connection: PgPool, filter: &String) -> ApiResponse<Vec<TaxAdjustmentRequests>> {
        match TaxAdjustmentRequestsRepository::get_by_filter(connection, filter).await {
            Ok(entity) => ApiResponse::success(entity),
            Err(e) => ApiResponse::error(e.to_string()),
        }
    }

    async fn add(connection: PgPool, entity: &TaxAdjustmentRequests) -> ApiResponse<bool> {
        match TaxAdjustmentRequestsRepository::add(connection, entity).await {
            Ok(entity) => ApiResponse::success(entity),
            Err(e) => ApiResponse::error(e.to_string()),
        }
    }

    async fn update(connection: PgPool, entity: &TaxAdjustmentRequests) -> ApiResponse<bool> {
        match TaxAdjustmentRequestsRepository::update(connection, entity).await {
            Ok(entity) => ApiResponse::success(entity),
            Err(e) => ApiResponse::error(e.to_string()),
        }
    }

    async fn delete(connection: PgPool, id: &String) -> ApiResponse<bool> {
        match TaxAdjustmentRequestsRepository::delete(connection, id).await {
            Ok(entity) => ApiResponse::success(entity),
            Err(e) => ApiResponse::error(e.to_string()),
        }
    }
}
