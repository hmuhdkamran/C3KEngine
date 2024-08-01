pub use sqlx::PgPool;

use crate::{
    interfaces::{irepository::IRepository, iservice::IService},
    models::{response::ApiResponse, roles::role::Role},
    repositories::roles::role_repository::RoleRepository,
};

pub struct RoleService {}

impl IService<Role> for RoleService {
    async fn get_all(connection: PgPool) -> ApiResponse<Vec<Role>> {
        match RoleRepository::get_all(connection).await {
            Ok(entity) => ApiResponse::success(entity),
            Err(e) => ApiResponse::error(e.to_string()),
        }
    }

    async fn get_by_filter(connection: PgPool, filter: &String) -> ApiResponse<Vec<Role>> {
        match RoleRepository::get_by_filter(connection, filter).await {
            Ok(entity) => ApiResponse::success(entity),
            Err(e) => ApiResponse::error(e.to_string()),
        }
    }

    async fn add(connection: PgPool, entity: &Role) -> ApiResponse<bool> {
        match RoleRepository::add(connection, entity).await {
            Ok(entity) => ApiResponse::success(entity),
            Err(e) => ApiResponse::error(e.to_string()),
        }
    }

    async fn update(connection: PgPool, entity: &Role) -> ApiResponse<bool> {
        match RoleRepository::update(connection, entity).await {
            Ok(entity) => ApiResponse::success(entity),
            Err(e) => ApiResponse::error(e.to_string()),
        }
    }

    async fn delete(connection: PgPool, id: &String) -> ApiResponse<bool> {
        match RoleRepository::delete(connection, id).await {
            Ok(entity) => ApiResponse::success(entity),
            Err(e) => ApiResponse::error(e.to_string()),
        }
    }
}
