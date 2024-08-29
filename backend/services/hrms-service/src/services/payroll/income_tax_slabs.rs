use crate::{
    models::payroll::income_tax_slabs::IncomeTaxSlabs,
    repositories::payroll::income_tax_slabs::IncomeTaxSlabsRepository,
};
use c3k_common::{
    interfaces::{irepository::IRepository, iservice::IService},
    models::response::ApiResponse,
};
pub use sqlx::PgPool;

pub struct IncomeTaxSlabsService {}

impl IService<IncomeTaxSlabs> for IncomeTaxSlabsService {
    async fn get_all(connection: PgPool) -> ApiResponse<Vec<IncomeTaxSlabs>> {
        match IncomeTaxSlabsRepository::get_all(connection).await {
            Ok(entity) => ApiResponse::success(entity),
            Err(e) => ApiResponse::error(e.to_string()),
        }
    }

    async fn get_by_filter(connection: PgPool, filter: &String) -> ApiResponse<Vec<IncomeTaxSlabs>> {
        match IncomeTaxSlabsRepository::get_by_filter(connection, filter).await {
            Ok(entity) => ApiResponse::success(entity),
            Err(e) => ApiResponse::error(e.to_string()),
        }
    }

    async fn add(connection: PgPool, entity: &IncomeTaxSlabs) -> ApiResponse<bool> {
        match IncomeTaxSlabsRepository::add(connection, entity).await {
            Ok(entity) => ApiResponse::success(entity),
            Err(e) => ApiResponse::error(e.to_string()),
        }
    }

    async fn update(connection: PgPool, entity: &IncomeTaxSlabs) -> ApiResponse<bool> {
        match IncomeTaxSlabsRepository::update(connection, entity).await {
            Ok(entity) => ApiResponse::success(entity),
            Err(e) => ApiResponse::error(e.to_string()),
        }
    }

    async fn delete(connection: PgPool, id: &String) -> ApiResponse<bool> {
        match IncomeTaxSlabsRepository::delete(connection, id).await {
            Ok(entity) => ApiResponse::success(entity),
            Err(e) => ApiResponse::error(e.to_string()),
        }
    }
}
