use crate::{
    models::attendance::gazetted_holidays::GazettedHolidays,
    repositories::attendance::gazetted_holidays::GazettedHolidaysRepository,
};
use c3k_common::{
    interfaces::{irepository::IRepository, iservice::IService},
    models::response::ApiResponse,
};
pub use sqlx::PgPool;

pub struct GazettedHolidaysService {}

impl IService<GazettedHolidays> for GazettedHolidaysService {
    async fn get_all(connection: PgPool) -> ApiResponse<Vec<GazettedHolidays>> {
        match GazettedHolidaysRepository::get_all(connection).await {
            Ok(entity) => ApiResponse::success(entity),
            Err(e) => ApiResponse::error(e.to_string()),
        }
    }

    async fn get_by_filter(connection: PgPool, filter: &String) -> ApiResponse<Vec<GazettedHolidays>> {
        match GazettedHolidaysRepository::get_by_filter(connection, filter).await {
            Ok(entity) => ApiResponse::success(entity),
            Err(e) => ApiResponse::error(e.to_string()),
        }
    }

    async fn add(connection: PgPool, entity: &GazettedHolidays) -> ApiResponse<bool> {
        match GazettedHolidaysRepository::add(connection, entity).await {
            Ok(entity) => ApiResponse::success(entity),
            Err(e) => ApiResponse::error(e.to_string()),
        }
    }

    async fn update(connection: PgPool, entity: &GazettedHolidays) -> ApiResponse<bool> {
        match GazettedHolidaysRepository::update(connection, entity).await {
            Ok(entity) => ApiResponse::success(entity),
            Err(e) => ApiResponse::error(e.to_string()),
        }
    }

    async fn delete(connection: PgPool, id: &String) -> ApiResponse<bool> {
        match GazettedHolidaysRepository::delete(connection, id).await {
            Ok(entity) => ApiResponse::success(entity),
            Err(e) => ApiResponse::error(e.to_string()),
        }
    }
}
