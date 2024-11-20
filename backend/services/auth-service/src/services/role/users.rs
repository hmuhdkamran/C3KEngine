use crate::{models::role::users::Users, repositories::role::users::UsersRepository};
use c3k_common::{
    interfaces::{irepository::IRepository, iservice::IService},
    models::{config::app_config::get_config, response::ApiResponse},
    utilities::security_utils::SecurityUtils,
};
pub use sqlx::PgPool;

pub struct UsersService {}

impl IService<Users> for UsersService {
    async fn get_all(connection: PgPool) -> ApiResponse<Vec<Users>> {
        match UsersRepository::get_all(connection).await {
            Ok(entity) => ApiResponse::success(entity),
            Err(e) => ApiResponse::error(e.to_string()),
        }
    }

    async fn get_by_filter(connection: PgPool, filter: &String) -> ApiResponse<Vec<Users>> {
        match UsersRepository::get_by_filter(connection, filter).await {
            Ok(entity) => ApiResponse::success(entity),
            Err(e) => ApiResponse::error(e.to_string()),
        }
    }

    async fn add(connection: PgPool, entity: &Users) -> ApiResponse<bool> {
        let config = match get_config() {
            Some(cfg) => cfg,
            None => {
                return ApiResponse::error("Configuration error".to_string());
            }
        };

        let (hash, salt) = SecurityUtils::encrypt_password(
            &entity.password,
            config.token_provider.salt_length,
            &config.token_provider.token_security_algorithm,
        )
        .expect("Failed to encrypt password");

        let new_entity = Users {
            salt,
            password: hash,
            ..entity.clone()
        };

        match UsersRepository::add(connection, &new_entity).await {
            Ok(entity) => ApiResponse::success(entity),
            Err(e) => ApiResponse::error(e.to_string()),
        }
    }

    async fn update(connection: PgPool, entity: &Users) -> ApiResponse<bool> {
        match UsersRepository::update(connection, entity).await {
            Ok(entity) => ApiResponse::success(entity),
            Err(e) => ApiResponse::error(e.to_string()),
        }
    }

    async fn delete(connection: PgPool, id: &String) -> ApiResponse<bool> {
        match UsersRepository::delete(connection, id).await {
            Ok(entity) => ApiResponse::success(entity),
            Err(e) => ApiResponse::error(e.to_string()),
        }
    }
}
