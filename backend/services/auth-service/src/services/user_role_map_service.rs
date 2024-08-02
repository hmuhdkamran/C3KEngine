pub use sqlx::PgPool;

use crate::{
    models::roles::user_role_map::UserRoleMap,
    repositories::roles::user_role_map_repository::UserRoleMapRepository,
};
use c3k_common::{
    interfaces::{irepository::IRepository, iservice::IService},
    models::response::ApiResponse,
};

pub struct UserRoleMapService {}

impl IService<UserRoleMap> for UserRoleMapService {
    async fn get_all(connection: PgPool) -> ApiResponse<Vec<UserRoleMap>> {
        match UserRoleMapRepository::get_all(connection).await {
            Ok(entity) => ApiResponse::success(entity),
            Err(e) => ApiResponse::error(e.to_string()),
        }
    }

    async fn get_by_filter(connection: PgPool, filter: &String) -> ApiResponse<Vec<UserRoleMap>> {
        match UserRoleMapRepository::get_by_filter(connection, filter).await {
            Ok(entity) => ApiResponse::success(entity),
            Err(e) => ApiResponse::error(e.to_string()),
        }
    }

    async fn add(connection: PgPool, entity: &UserRoleMap) -> ApiResponse<bool> {
        match UserRoleMapRepository::add(connection, entity).await {
            Ok(entity) => ApiResponse::success(entity),
            Err(e) => ApiResponse::error(e.to_string()),
        }
    }

    async fn update(connection: PgPool, entity: &UserRoleMap) -> ApiResponse<bool> {
        match UserRoleMapRepository::update(connection, entity).await {
            Ok(entity) => ApiResponse::success(entity),
            Err(e) => ApiResponse::error(e.to_string()),
        }
    }

    async fn delete(connection: PgPool, id: &String) -> ApiResponse<bool> {
        match UserRoleMapRepository::delete(connection, id).await {
            Ok(entity) => ApiResponse::success(entity),
            Err(e) => ApiResponse::error(e.to_string()),
        }
    }
}
