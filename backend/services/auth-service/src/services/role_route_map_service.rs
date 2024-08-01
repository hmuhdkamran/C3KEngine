pub use sqlx::PgPool;

use crate::{
    models::roles::role_route_map::RoleRouteMap,
    repositories::roles::role_route_map_repository::RoleRouteMapRepository,
};
use c3k_common::{
    interfaces::{irepository::IRepository, iservice::IService},
    models::response::ApiResponse,
};

pub struct RoleRouteMapService {}

impl IService<RoleRouteMap> for RoleRouteMapService {
    async fn get_all(connection: PgPool) -> ApiResponse<Vec<RoleRouteMap>> {
        match RoleRouteMapRepository::get_all(connection).await {
            Ok(entity) => ApiResponse::success(entity),
            Err(e) => ApiResponse::error(e.to_string()),
        }
    }

    async fn get_by_filter(connection: PgPool, filter: &String) -> ApiResponse<Vec<RoleRouteMap>> {
        match RoleRouteMapRepository::get_by_filter(connection, filter).await {
            Ok(entity) => ApiResponse::success(entity),
            Err(e) => ApiResponse::error(e.to_string()),
        }
    }

    async fn add(connection: PgPool, entity: &RoleRouteMap) -> ApiResponse<bool> {
        match RoleRouteMapRepository::add(connection, entity).await {
            Ok(entity) => ApiResponse::success(entity),
            Err(e) => ApiResponse::error(e.to_string()),
        }
    }

    async fn update(connection: PgPool, entity: &RoleRouteMap) -> ApiResponse<bool> {
        match RoleRouteMapRepository::update(connection, entity).await {
            Ok(entity) => ApiResponse::success(entity),
            Err(e) => ApiResponse::error(e.to_string()),
        }
    }

    async fn delete(connection: PgPool, id: &String) -> ApiResponse<bool> {
        match RoleRouteMapRepository::delete(connection, id).await {
            Ok(entity) => ApiResponse::success(entity),
            Err(e) => ApiResponse::error(e.to_string()),
        }
    }
}
