use crate::{
    models::payroll::loan_markup_rates::LoanMarkupRates,
    repositories::payroll::loan_markup_rates::LoanMarkupRatesRepository,
};
use c3k_common::{
    interfaces::{irepository::IRepository, iservice::IService},
    models::response::ApiResponse,
};
pub use sqlx::PgPool;

pub struct LoanMarkupRatesService {}

impl IService<LoanMarkupRates> for LoanMarkupRatesService {
    async fn get_all(connection: PgPool) -> ApiResponse<Vec<LoanMarkupRates>> {
        match LoanMarkupRatesRepository::get_all(connection).await {
            Ok(entity) => ApiResponse::success(entity),
            Err(e) => ApiResponse::error(e.to_string()),
        }
    }

    async fn get_by_filter(connection: PgPool, filter: &String) -> ApiResponse<Vec<LoanMarkupRates>> {
        match LoanMarkupRatesRepository::get_by_filter(connection, filter).await {
            Ok(entity) => ApiResponse::success(entity),
            Err(e) => ApiResponse::error(e.to_string()),
        }
    }

    async fn add(connection: PgPool, entity: &LoanMarkupRates) -> ApiResponse<bool> {
        match LoanMarkupRatesRepository::add(connection, entity).await {
            Ok(entity) => ApiResponse::success(entity),
            Err(e) => ApiResponse::error(e.to_string()),
        }
    }

    async fn update(connection: PgPool, entity: &LoanMarkupRates) -> ApiResponse<bool> {
        match LoanMarkupRatesRepository::update(connection, entity).await {
            Ok(entity) => ApiResponse::success(entity),
            Err(e) => ApiResponse::error(e.to_string()),
        }
    }

    async fn delete(connection: PgPool, id: &String) -> ApiResponse<bool> {
        match LoanMarkupRatesRepository::delete(connection, id).await {
            Ok(entity) => ApiResponse::success(entity),
            Err(e) => ApiResponse::error(e.to_string()),
        }
    }
}
