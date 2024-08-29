use crate::{
    models::employee::employee_experiences::EmployeeExperiences,
    repositories::employee::employee_experiences::EmployeeExperiencesRepository,
};
use c3k_common::{
    interfaces::{irepository::IRepository, iservice::IService},
    models::response::ApiResponse,
};
pub use sqlx::PgPool;

pub struct EmployeeExperiencesService {}

impl IService<EmployeeExperiences> for EmployeeExperiencesService {
    async fn get_all(connection: PgPool) -> ApiResponse<Vec<EmployeeExperiences>> {
        match EmployeeExperiencesRepository::get_all(connection).await {
            Ok(entity) => ApiResponse::success(entity),
            Err(e) => ApiResponse::error(e.to_string()),
        }
    }

    async fn get_by_filter(connection: PgPool, filter: &String) -> ApiResponse<Vec<EmployeeExperiences>> {
        match EmployeeExperiencesRepository::get_by_filter(connection, filter).await {
            Ok(entity) => ApiResponse::success(entity),
            Err(e) => ApiResponse::error(e.to_string()),
        }
    }

    async fn add(connection: PgPool, entity: &EmployeeExperiences) -> ApiResponse<bool> {
        match EmployeeExperiencesRepository::add(connection, entity).await {
            Ok(entity) => ApiResponse::success(entity),
            Err(e) => ApiResponse::error(e.to_string()),
        }
    }

    async fn update(connection: PgPool, entity: &EmployeeExperiences) -> ApiResponse<bool> {
        match EmployeeExperiencesRepository::update(connection, entity).await {
            Ok(entity) => ApiResponse::success(entity),
            Err(e) => ApiResponse::error(e.to_string()),
        }
    }

    async fn delete(connection: PgPool, id: &String) -> ApiResponse<bool> {
        match EmployeeExperiencesRepository::delete(connection, id).await {
            Ok(entity) => ApiResponse::success(entity),
            Err(e) => ApiResponse::error(e.to_string()),
        }
    }
}
