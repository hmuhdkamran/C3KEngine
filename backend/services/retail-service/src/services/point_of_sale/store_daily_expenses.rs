use crate::{
    models::point_of_sale::store_daily_expenses::StoreDailyExpenses,
    repositories::point_of_sale::store_daily_expenses::StoreDailyExpensesRepository,
};
use c3k_common::{
    interfaces::{irepository::IRepository, iservice::IService},
    models::response::ApiResponse,
};
pub use sqlx::PgPool;

pub struct StoreDailyExpensesService {}

impl IService<StoreDailyExpenses> for StoreDailyExpensesService {
    async fn get_all(connection: PgPool) -> ApiResponse<Vec<StoreDailyExpenses>> {
        match StoreDailyExpensesRepository::get_all(connection).await {
            Ok(entity) => ApiResponse::success(entity),
            Err(e) => ApiResponse::error(e.to_string()),
        }
    }

    async fn get_by_filter(connection: PgPool, filter: &String) -> ApiResponse<Vec<StoreDailyExpenses>> {
        match StoreDailyExpensesRepository::get_by_filter(connection, filter).await {
            Ok(entity) => ApiResponse::success(entity),
            Err(e) => ApiResponse::error(e.to_string()),
        }
    }

    async fn add(connection: PgPool, entity: &StoreDailyExpenses) -> ApiResponse<bool> {
        match StoreDailyExpensesRepository::add(connection, entity).await {
            Ok(entity) => ApiResponse::success(entity),
            Err(e) => ApiResponse::error(e.to_string()),
        }
    }

    async fn update(connection: PgPool, entity: &StoreDailyExpenses) -> ApiResponse<bool> {
        match StoreDailyExpensesRepository::update(connection, entity).await {
            Ok(entity) => ApiResponse::success(entity),
            Err(e) => ApiResponse::error(e.to_string()),
        }
    }

    async fn delete(connection: PgPool, id: &String) -> ApiResponse<bool> {
        match StoreDailyExpensesRepository::delete(connection, id).await {
            Ok(entity) => ApiResponse::success(entity),
            Err(e) => ApiResponse::error(e.to_string()),
        }
    }
}
