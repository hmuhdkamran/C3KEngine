use crate::{
    models::payroll::loan_installment_plans::LoanInstallmentPlans,
    repositories::payroll::loan_installment_plans::LoanInstallmentPlansRepository,
};
use c3k_common::{
    interfaces::{irepository::IRepository, iservice::IService},
    models::response::ApiResponse,
};
pub use sqlx::PgPool;

pub struct LoanInstallmentPlansService {}

impl IService<LoanInstallmentPlans> for LoanInstallmentPlansService {
    async fn get_all(connection: PgPool) -> ApiResponse<Vec<LoanInstallmentPlans>> {
        match LoanInstallmentPlansRepository::get_all(connection).await {
            Ok(entity) => ApiResponse::success(entity),
            Err(e) => ApiResponse::error(e.to_string()),
        }
    }

    async fn get_by_filter(connection: PgPool, filter: &String) -> ApiResponse<Vec<LoanInstallmentPlans>> {
        match LoanInstallmentPlansRepository::get_by_filter(connection, filter).await {
            Ok(entity) => ApiResponse::success(entity),
            Err(e) => ApiResponse::error(e.to_string()),
        }
    }

    async fn add(connection: PgPool, entity: &LoanInstallmentPlans) -> ApiResponse<bool> {
        match LoanInstallmentPlansRepository::add(connection, entity).await {
            Ok(entity) => ApiResponse::success(entity),
            Err(e) => ApiResponse::error(e.to_string()),
        }
    }

    async fn update(connection: PgPool, entity: &LoanInstallmentPlans) -> ApiResponse<bool> {
        match LoanInstallmentPlansRepository::update(connection, entity).await {
            Ok(entity) => ApiResponse::success(entity),
            Err(e) => ApiResponse::error(e.to_string()),
        }
    }

    async fn delete(connection: PgPool, id: &String) -> ApiResponse<bool> {
        match LoanInstallmentPlansRepository::delete(connection, id).await {
            Ok(entity) => ApiResponse::success(entity),
            Err(e) => ApiResponse::error(e.to_string()),
        }
    }
}
