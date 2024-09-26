use crate::{
    models::point_of_sale::payment_terms::PaymentTerms,
    repositories::point_of_sale::payment_terms::PaymentTermsRepository,
};
use c3k_common::{
    interfaces::{irepository::IRepository, iservice::IService},
    models::response::ApiResponse,
};
pub use sqlx::PgPool;

pub struct PaymentTermsService {}

impl IService<PaymentTerms> for PaymentTermsService {
    async fn get_all(connection: PgPool) -> ApiResponse<Vec<PaymentTerms>> {
        match PaymentTermsRepository::get_all(connection).await {
            Ok(entity) => ApiResponse::success(entity),
            Err(e) => ApiResponse::error(e.to_string()),
        }
    }

    async fn get_by_filter(connection: PgPool, filter: &String) -> ApiResponse<Vec<PaymentTerms>> {
        match PaymentTermsRepository::get_by_filter(connection, filter).await {
            Ok(entity) => ApiResponse::success(entity),
            Err(e) => ApiResponse::error(e.to_string()),
        }
    }

    async fn add(connection: PgPool, entity: &PaymentTerms) -> ApiResponse<bool> {
        match PaymentTermsRepository::add(connection, entity).await {
            Ok(entity) => ApiResponse::success(entity),
            Err(e) => ApiResponse::error(e.to_string()),
        }
    }

    async fn update(connection: PgPool, entity: &PaymentTerms) -> ApiResponse<bool> {
        match PaymentTermsRepository::update(connection, entity).await {
            Ok(entity) => ApiResponse::success(entity),
            Err(e) => ApiResponse::error(e.to_string()),
        }
    }

    async fn delete(connection: PgPool, id: &String) -> ApiResponse<bool> {
        match PaymentTermsRepository::delete(connection, id).await {
            Ok(entity) => ApiResponse::success(entity),
            Err(e) => ApiResponse::error(e.to_string()),
        }
    }
}
