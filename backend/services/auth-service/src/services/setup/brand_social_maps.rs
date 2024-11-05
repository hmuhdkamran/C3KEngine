use crate::{
    models::setup::brand_social_maps::BrandSocialMaps,
    repositories::setup::brand_social_maps::BrandSocialMapsRepository,
};
use c3k_common::{
    interfaces::{irepository::IRepository, iservice::IService},
    models::response::ApiResponse,
};
pub use sqlx::PgPool;

pub struct BrandSocialMapsService {}

impl IService<BrandSocialMaps> for BrandSocialMapsService {
    async fn get_all(connection: PgPool) -> ApiResponse<Vec<BrandSocialMaps>> {
        match BrandSocialMapsRepository::get_all(connection).await {
            Ok(entity) => ApiResponse::success(entity),
            Err(e) => ApiResponse::error(e.to_string()),
        }
    }

    async fn get_by_filter(connection: PgPool, filter: &String) -> ApiResponse<Vec<BrandSocialMaps>> {
        match BrandSocialMapsRepository::get_by_filter(connection, filter).await {
            Ok(entity) => ApiResponse::success(entity),
            Err(e) => ApiResponse::error(e.to_string()),
        }
    }

    async fn add(connection: PgPool, entity: &BrandSocialMaps) -> ApiResponse<bool> {
        match BrandSocialMapsRepository::add(connection, entity).await {
            Ok(entity) => ApiResponse::success(entity),
            Err(e) => ApiResponse::error(e.to_string()),
        }
    }

    async fn update(connection: PgPool, entity: &BrandSocialMaps) -> ApiResponse<bool> {
        match BrandSocialMapsRepository::update(connection, entity).await {
            Ok(entity) => ApiResponse::success(entity),
            Err(e) => ApiResponse::error(e.to_string()),
        }
    }

    async fn delete(connection: PgPool, id: &String) -> ApiResponse<bool> {
        match BrandSocialMapsRepository::delete(connection, id).await {
            Ok(entity) => ApiResponse::success(entity),
            Err(e) => ApiResponse::error(e.to_string()),
        }
    }
}
