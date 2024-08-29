use crate::{
    models::employee::employee_bank_infos::EmployeeBankInfos,
    repositories::employee::employee_bank_infos::EmployeeBankInfosRepository,
};
use c3k_common::{
    interfaces::{irepository::IRepository, iservice::IService},
    models::response::ApiResponse,
};
pub use sqlx::PgPool;

pub struct EmployeeBankInfosService {}

impl IService<EmployeeBankInfos> for EmployeeBankInfosService {
    async fn get_all(connection: PgPool) -> ApiResponse<Vec<EmployeeBankInfos>> {
        match EmployeeBankInfosRepository::get_all(connection).await {
            Ok(entity) => ApiResponse::success(entity),
            Err(e) => ApiResponse::error(e.to_string()),
        }
    }

    async fn get_by_filter(connection: PgPool, filter: &String) -> ApiResponse<Vec<EmployeeBankInfos>> {
        match EmployeeBankInfosRepository::get_by_filter(connection, filter).await {
            Ok(entity) => ApiResponse::success(entity),
            Err(e) => ApiResponse::error(e.to_string()),
        }
    }

    async fn add(connection: PgPool, entity: &EmployeeBankInfos) -> ApiResponse<bool> {
        match EmployeeBankInfosRepository::add(connection, entity).await {
            Ok(entity) => ApiResponse::success(entity),
            Err(e) => ApiResponse::error(e.to_string()),
        }
    }

    async fn update(connection: PgPool, entity: &EmployeeBankInfos) -> ApiResponse<bool> {
        match EmployeeBankInfosRepository::update(connection, entity).await {
            Ok(entity) => ApiResponse::success(entity),
            Err(e) => ApiResponse::error(e.to_string()),
        }
    }

    async fn delete(connection: PgPool, id: &String) -> ApiResponse<bool> {
        match EmployeeBankInfosRepository::delete(connection, id).await {
            Ok(entity) => ApiResponse::success(entity),
            Err(e) => ApiResponse::error(e.to_string()),
        }
    }
}
