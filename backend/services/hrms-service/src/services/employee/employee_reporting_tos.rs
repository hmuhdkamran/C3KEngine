use crate::{
    models::employee::employee_reporting_tos::EmployeeReportingTos,
    repositories::employee::employee_reporting_tos::EmployeeReportingTosRepository,
};
use c3k_common::{
    interfaces::{irepository::IRepository, iservice::IService},
    models::response::ApiResponse,
};
pub use sqlx::PgPool;

pub struct EmployeeReportingTosService {}

impl IService<EmployeeReportingTos> for EmployeeReportingTosService {
    async fn get_all(connection: PgPool) -> ApiResponse<Vec<EmployeeReportingTos>> {
        match EmployeeReportingTosRepository::get_all(connection).await {
            Ok(entity) => ApiResponse::success(entity),
            Err(e) => ApiResponse::error(e.to_string()),
        }
    }

    async fn get_by_filter(connection: PgPool, filter: &String) -> ApiResponse<Vec<EmployeeReportingTos>> {
        match EmployeeReportingTosRepository::get_by_filter(connection, filter).await {
            Ok(entity) => ApiResponse::success(entity),
            Err(e) => ApiResponse::error(e.to_string()),
        }
    }

    async fn add(connection: PgPool, entity: &EmployeeReportingTos) -> ApiResponse<bool> {
        match EmployeeReportingTosRepository::add(connection, entity).await {
            Ok(entity) => ApiResponse::success(entity),
            Err(e) => ApiResponse::error(e.to_string()),
        }
    }

    async fn update(connection: PgPool, entity: &EmployeeReportingTos) -> ApiResponse<bool> {
        match EmployeeReportingTosRepository::update(connection, entity).await {
            Ok(entity) => ApiResponse::success(entity),
            Err(e) => ApiResponse::error(e.to_string()),
        }
    }

    async fn delete(connection: PgPool, id: &String) -> ApiResponse<bool> {
        match EmployeeReportingTosRepository::delete(connection, id).await {
            Ok(entity) => ApiResponse::success(entity),
            Err(e) => ApiResponse::error(e.to_string()),
        }
    }
}
