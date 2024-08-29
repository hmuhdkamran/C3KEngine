use crate::{
    models::recruitment::recruitment_job_statuses::RecruitmentJobStatuses,
    repositories::recruitment::recruitment_job_statuses::RecruitmentJobStatusesRepository,
};
use c3k_common::{
    interfaces::{irepository::IRepository, iservice::IService},
    models::response::ApiResponse,
};
pub use sqlx::PgPool;

pub struct RecruitmentJobStatusesService {}

impl IService<RecruitmentJobStatuses> for RecruitmentJobStatusesService {
    async fn get_all(connection: PgPool) -> ApiResponse<Vec<RecruitmentJobStatuses>> {
        match RecruitmentJobStatusesRepository::get_all(connection).await {
            Ok(entity) => ApiResponse::success(entity),
            Err(e) => ApiResponse::error(e.to_string()),
        }
    }

    async fn get_by_filter(connection: PgPool, filter: &String) -> ApiResponse<Vec<RecruitmentJobStatuses>> {
        match RecruitmentJobStatusesRepository::get_by_filter(connection, filter).await {
            Ok(entity) => ApiResponse::success(entity),
            Err(e) => ApiResponse::error(e.to_string()),
        }
    }

    async fn add(connection: PgPool, entity: &RecruitmentJobStatuses) -> ApiResponse<bool> {
        match RecruitmentJobStatusesRepository::add(connection, entity).await {
            Ok(entity) => ApiResponse::success(entity),
            Err(e) => ApiResponse::error(e.to_string()),
        }
    }

    async fn update(connection: PgPool, entity: &RecruitmentJobStatuses) -> ApiResponse<bool> {
        match RecruitmentJobStatusesRepository::update(connection, entity).await {
            Ok(entity) => ApiResponse::success(entity),
            Err(e) => ApiResponse::error(e.to_string()),
        }
    }

    async fn delete(connection: PgPool, id: &String) -> ApiResponse<bool> {
        match RecruitmentJobStatusesRepository::delete(connection, id).await {
            Ok(entity) => ApiResponse::success(entity),
            Err(e) => ApiResponse::error(e.to_string()),
        }
    }
}
