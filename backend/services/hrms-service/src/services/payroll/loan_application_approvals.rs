use crate::{
    models::payroll::loan_application_approvals::LoanApplicationApprovals,
    repositories::payroll::loan_application_approvals::LoanApplicationApprovalsRepository,
};
use c3k_common::{
    interfaces::{irepository::IRepository, iservice::IService},
    models::response::ApiResponse,
};
pub use sqlx::PgPool;

pub struct LoanApplicationApprovalsService {}

impl IService<LoanApplicationApprovals> for LoanApplicationApprovalsService {
    async fn get_all(connection: PgPool) -> ApiResponse<Vec<LoanApplicationApprovals>> {
        match LoanApplicationApprovalsRepository::get_all(connection).await {
            Ok(entity) => ApiResponse::success(entity),
            Err(e) => ApiResponse::error(e.to_string()),
        }
    }

    async fn get_by_filter(connection: PgPool, filter: &String) -> ApiResponse<Vec<LoanApplicationApprovals>> {
        match LoanApplicationApprovalsRepository::get_by_filter(connection, filter).await {
            Ok(entity) => ApiResponse::success(entity),
            Err(e) => ApiResponse::error(e.to_string()),
        }
    }

    async fn add(connection: PgPool, entity: &LoanApplicationApprovals) -> ApiResponse<bool> {
        match LoanApplicationApprovalsRepository::add(connection, entity).await {
            Ok(entity) => ApiResponse::success(entity),
            Err(e) => ApiResponse::error(e.to_string()),
        }
    }

    async fn update(connection: PgPool, entity: &LoanApplicationApprovals) -> ApiResponse<bool> {
        match LoanApplicationApprovalsRepository::update(connection, entity).await {
            Ok(entity) => ApiResponse::success(entity),
            Err(e) => ApiResponse::error(e.to_string()),
        }
    }

    async fn delete(connection: PgPool, id: &String) -> ApiResponse<bool> {
        match LoanApplicationApprovalsRepository::delete(connection, id).await {
            Ok(entity) => ApiResponse::success(entity),
            Err(e) => ApiResponse::error(e.to_string()),
        }
    }
}
