use crate::{
    models::roles::user_product_map::UserProductMap,
    repositories::roles::user_product_map_repository::UserProductMapRepository,
};
use c3k_common::{
    interfaces::{irepository::IRepository, iservice::IService},
    models::response::ApiResponse,
};
pub use sqlx::PgPool;

pub struct UserProductMapService {}

impl IService<UserProductMap> for UserProductMapService {
    async fn get_all(connection: PgPool) -> ApiResponse<Vec<UserProductMap>> {
        match UserProductMapRepository::get_all(connection).await {
            Ok(entity) => ApiResponse::success(entity),
            Err(e) => ApiResponse::error(e.to_string()),
        }
    }

    async fn get_by_filter(
        connection: PgPool,
        filter: &String,
    ) -> ApiResponse<Vec<UserProductMap>> {
        match UserProductMapRepository::get_by_filter(connection, filter).await {
            Ok(entity) => ApiResponse::success(entity),
            Err(e) => ApiResponse::error(e.to_string()),
        }
    }

    async fn add(connection: PgPool, entity: &UserProductMap) -> ApiResponse<bool> {
        match UserProductMapRepository::add(connection, entity).await {
            Ok(entity) => ApiResponse::success(entity),
            Err(e) => ApiResponse::error(e.to_string()),
        }
    }

    async fn update(connection: PgPool, entity: &UserProductMap) -> ApiResponse<bool> {
        match UserProductMapRepository::update(connection, entity).await {
            Ok(entity) => ApiResponse::success(entity),
            Err(e) => ApiResponse::error(e.to_string()),
        }
    }

    async fn delete(connection: PgPool, id: &String) -> ApiResponse<bool> {
        match UserProductMapRepository::delete(connection, id).await {
            Ok(entity) => ApiResponse::success(entity),
            Err(e) => ApiResponse::error(e.to_string()),
        }
    }
}
