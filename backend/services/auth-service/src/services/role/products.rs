use crate::{models::role::products::Products, repositories::role::products::ProductsRepository};
use c3k_common::{
    interfaces::{irepository::IRepository, iservice::IService},
    models::{
        auth::{Auth, UserProducts},
        response::ApiResponse,
    },
};
pub use sqlx::PgPool;

pub struct ProductsService {}

impl IService<Products> for ProductsService {
    async fn get_all(connection: PgPool) -> ApiResponse<Vec<Products>> {
        match ProductsRepository::get_all(connection).await {
            Ok(entity) => ApiResponse::success(entity),
            Err(e) => ApiResponse::error(e.to_string()),
        }
    }

    async fn get_by_filter(connection: PgPool, filter: &String) -> ApiResponse<Vec<Products>> {
        match ProductsRepository::get_by_filter(connection, filter).await {
            Ok(entity) => ApiResponse::success(entity),
            Err(e) => ApiResponse::error(e.to_string()),
        }
    }

    async fn add(connection: PgPool, entity: &Products) -> ApiResponse<bool> {
        match ProductsRepository::add(connection, entity).await {
            Ok(entity) => ApiResponse::success(entity),
            Err(e) => ApiResponse::error(e.to_string()),
        }
    }

    async fn update(connection: PgPool, entity: &Products) -> ApiResponse<bool> {
        match ProductsRepository::update(connection, entity).await {
            Ok(entity) => ApiResponse::success(entity),
            Err(e) => ApiResponse::error(e.to_string()),
        }
    }

    async fn delete(connection: PgPool, id: &String) -> ApiResponse<bool> {
        match ProductsRepository::delete(connection, id).await {
            Ok(entity) => ApiResponse::success(entity),
            Err(e) => ApiResponse::error(e.to_string()),
        }
    }
}

impl ProductsService {
    pub async fn get_products(connection: PgPool, username: &String) -> ApiResponse<Vec<UserProducts>> {
        match ProductsRepository::get_products(connection, username).await {
            Ok(entity) => ApiResponse::success(entity),
            Err(e) => ApiResponse::error(e.to_string()),
        }
    }

    pub async fn get_claims(
        connection: PgPool,
        username: &String,
        product: &String,
    ) -> ApiResponse<Vec<Auth>> {
        match ProductsRepository::get_claims(connection, username, product).await {
            Ok(entity) => ApiResponse::success(entity),
            Err(e) => ApiResponse::error(e.to_string()),
        }
    }
}
