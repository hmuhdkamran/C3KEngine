use crate::{
    models::employee::employee_document_apporovals::EmployeeDocumentApporovals,
    repositories::employee::employee_document_apporovals::EmployeeDocumentApporovalsRepository,
};
use c3k_common::{
    interfaces::{irepository::IRepository, iservice::IService},
    models::response::ApiResponse,
};
pub use sqlx::PgPool;

pub struct EmployeeDocumentApporovalsService {}

impl IService<EmployeeDocumentApporovals> for EmployeeDocumentApporovalsService {
    async fn get_all(connection: PgPool) -> ApiResponse<Vec<EmployeeDocumentApporovals>> {
        match EmployeeDocumentApporovalsRepository::get_all(connection).await {
            Ok(entity) => ApiResponse::success(entity),
            Err(e) => ApiResponse::error(e.to_string()),
        }
    }

    async fn get_by_filter(connection: PgPool, filter: &String) -> ApiResponse<Vec<EmployeeDocumentApporovals>> {
        match EmployeeDocumentApporovalsRepository::get_by_filter(connection, filter).await {
            Ok(entity) => ApiResponse::success(entity),
            Err(e) => ApiResponse::error(e.to_string()),
        }
    }

    async fn add(connection: PgPool, entity: &EmployeeDocumentApporovals) -> ApiResponse<bool> {
        match EmployeeDocumentApporovalsRepository::add(connection, entity).await {
            Ok(entity) => ApiResponse::success(entity),
            Err(e) => ApiResponse::error(e.to_string()),
        }
    }

    async fn update(connection: PgPool, entity: &EmployeeDocumentApporovals) -> ApiResponse<bool> {
        match EmployeeDocumentApporovalsRepository::update(connection, entity).await {
            Ok(entity) => ApiResponse::success(entity),
            Err(e) => ApiResponse::error(e.to_string()),
        }
    }

    async fn delete(connection: PgPool, id: &String) -> ApiResponse<bool> {
        match EmployeeDocumentApporovalsRepository::delete(connection, id).await {
            Ok(entity) => ApiResponse::success(entity),
            Err(e) => ApiResponse::error(e.to_string()),
        }
    }
}
