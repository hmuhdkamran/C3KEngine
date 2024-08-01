pub use sqlx::PgPool;

use crate::{
    interfaces::{irepository::IRepository, iservice::IService},
    models::{response::ApiResponse, roles::route::Route},
    repositories::roles::route_repository::RouteRepository,
};

pub struct RouteService {}

impl IService<Route> for RouteService {
    async fn get_all(connection: PgPool) -> ApiResponse<Vec<Route>> {
        match RouteRepository::get_all(connection).await {
            Ok(entity) => ApiResponse::success(entity),
            Err(e) => ApiResponse::error(e.to_string()),
        }
    }

    async fn get_by_filter(connection: PgPool, filter: &String) -> ApiResponse<Vec<Route>> {
        match RouteRepository::get_by_filter(connection, filter).await {
            Ok(entity) => ApiResponse::success(entity),
            Err(e) => ApiResponse::error(e.to_string()),
        }
    }

    async fn add(connection: PgPool, entity: &Route) -> ApiResponse<bool> {
        match RouteRepository::add(connection, entity).await {
            Ok(entity) => ApiResponse::success(entity),
            Err(e) => ApiResponse::error(e.to_string()),
        }
    }

    async fn update(connection: PgPool, entity: &Route) -> ApiResponse<bool> {
        match RouteRepository::update(connection, entity).await {
            Ok(entity) => ApiResponse::success(entity),
            Err(e) => ApiResponse::error(e.to_string()),
        }
    }

    async fn delete(connection: PgPool, id: &String) -> ApiResponse<bool> {
        match RouteRepository::delete(connection, id).await {
            Ok(entity) => ApiResponse::success(entity),
            Err(e) => ApiResponse::error(e.to_string()),
        }
    }
}
