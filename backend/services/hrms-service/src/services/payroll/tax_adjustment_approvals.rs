use crate::{
    models::payroll::tax_adjustment_approvals::TaxAdjustmentApprovals,
    repositories::payroll::tax_adjustment_approvals::TaxAdjustmentApprovalsRepository,
};
use c3k_common::{
    interfaces::{irepository::IRepository, iservice::IService},
    models::response::ApiResponse,
};
pub use sqlx::PgPool;

pub struct TaxAdjustmentApprovalsService {}

impl IService<TaxAdjustmentApprovals> for TaxAdjustmentApprovalsService {
    async fn get_all(connection: PgPool) -> ApiResponse<Vec<TaxAdjustmentApprovals>> {
        match TaxAdjustmentApprovalsRepository::get_all(connection).await {
            Ok(entity) => ApiResponse::success(entity),
            Err(e) => ApiResponse::error(e.to_string()),
        }
    }

    async fn get_by_filter(connection: PgPool, filter: &String) -> ApiResponse<Vec<TaxAdjustmentApprovals>> {
        match TaxAdjustmentApprovalsRepository::get_by_filter(connection, filter).await {
            Ok(entity) => ApiResponse::success(entity),
            Err(e) => ApiResponse::error(e.to_string()),
        }
    }

    async fn add(connection: PgPool, entity: &TaxAdjustmentApprovals) -> ApiResponse<bool> {
        match TaxAdjustmentApprovalsRepository::add(connection, entity).await {
            Ok(entity) => ApiResponse::success(entity),
            Err(e) => ApiResponse::error(e.to_string()),
        }
    }

    async fn update(connection: PgPool, entity: &TaxAdjustmentApprovals) -> ApiResponse<bool> {
        match TaxAdjustmentApprovalsRepository::update(connection, entity).await {
            Ok(entity) => ApiResponse::success(entity),
            Err(e) => ApiResponse::error(e.to_string()),
        }
    }

    async fn delete(connection: PgPool, id: &String) -> ApiResponse<bool> {
        match TaxAdjustmentApprovalsRepository::delete(connection, id).await {
            Ok(entity) => ApiResponse::success(entity),
            Err(e) => ApiResponse::error(e.to_string()),
        }
    }
}
