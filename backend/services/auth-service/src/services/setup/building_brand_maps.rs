use crate::{
    models::setup::building_brand_maps::BuildingBrandMaps,
    repositories::setup::building_brand_maps::BuildingBrandMapsRepository,
};
use c3k_common::{
    interfaces::{irepository::IRepository, iservice::IService},
    models::response::ApiResponse,
};
pub use sqlx::PgPool;

pub struct BuildingBrandMapsService {}

impl IService<BuildingBrandMaps> for BuildingBrandMapsService {
    async fn get_all(connection: PgPool) -> ApiResponse<Vec<BuildingBrandMaps>> {
        match BuildingBrandMapsRepository::get_all(connection).await {
            Ok(entity) => ApiResponse::success(entity),
            Err(e) => ApiResponse::error(e.to_string()),
        }
    }

    async fn get_by_filter(connection: PgPool, filter: &String) -> ApiResponse<Vec<BuildingBrandMaps>> {
        match BuildingBrandMapsRepository::get_by_filter(connection, filter).await {
            Ok(entity) => ApiResponse::success(entity),
            Err(e) => ApiResponse::error(e.to_string()),
        }
    }

    async fn add(connection: PgPool, entity: &BuildingBrandMaps) -> ApiResponse<bool> {
        match BuildingBrandMapsRepository::add(connection, entity).await {
            Ok(entity) => ApiResponse::success(entity),
            Err(e) => ApiResponse::error(e.to_string()),
        }
    }

    async fn update(connection: PgPool, entity: &BuildingBrandMaps) -> ApiResponse<bool> {
        match BuildingBrandMapsRepository::update(connection, entity).await {
            Ok(entity) => ApiResponse::success(entity),
            Err(e) => ApiResponse::error(e.to_string()),
        }
    }

    async fn delete(connection: PgPool, id: &String) -> ApiResponse<bool> {
        match BuildingBrandMapsRepository::delete(connection, id).await {
            Ok(entity) => ApiResponse::success(entity),
            Err(e) => ApiResponse::error(e.to_string()),
        }
    }
}
