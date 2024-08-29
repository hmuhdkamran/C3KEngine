use crate::{
    models::attendance::short_leave_apporvals::ShortLeaveApporvals,
    repositories::attendance::short_leave_apporvals::ShortLeaveApporvalsRepository,
};
use c3k_common::{
    interfaces::{irepository::IRepository, iservice::IService},
    models::response::ApiResponse,
};
pub use sqlx::PgPool;

pub struct ShortLeaveApporvalsService {}

impl IService<ShortLeaveApporvals> for ShortLeaveApporvalsService {
    async fn get_all(connection: PgPool) -> ApiResponse<Vec<ShortLeaveApporvals>> {
        match ShortLeaveApporvalsRepository::get_all(connection).await {
            Ok(entity) => ApiResponse::success(entity),
            Err(e) => ApiResponse::error(e.to_string()),
        }
    }

    async fn get_by_filter(connection: PgPool, filter: &String) -> ApiResponse<Vec<ShortLeaveApporvals>> {
        match ShortLeaveApporvalsRepository::get_by_filter(connection, filter).await {
            Ok(entity) => ApiResponse::success(entity),
            Err(e) => ApiResponse::error(e.to_string()),
        }
    }

    async fn add(connection: PgPool, entity: &ShortLeaveApporvals) -> ApiResponse<bool> {
        match ShortLeaveApporvalsRepository::add(connection, entity).await {
            Ok(entity) => ApiResponse::success(entity),
            Err(e) => ApiResponse::error(e.to_string()),
        }
    }

    async fn update(connection: PgPool, entity: &ShortLeaveApporvals) -> ApiResponse<bool> {
        match ShortLeaveApporvalsRepository::update(connection, entity).await {
            Ok(entity) => ApiResponse::success(entity),
            Err(e) => ApiResponse::error(e.to_string()),
        }
    }

    async fn delete(connection: PgPool, id: &String) -> ApiResponse<bool> {
        match ShortLeaveApporvalsRepository::delete(connection, id).await {
            Ok(entity) => ApiResponse::success(entity),
            Err(e) => ApiResponse::error(e.to_string()),
        }
    }
}
