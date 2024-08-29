use crate::{
    models::attendance::attendance_policies::AttendancePolicies,
    repositories::attendance::attendance_policies::AttendancePoliciesRepository,
};
use c3k_common::{
    interfaces::{irepository::IRepository, iservice::IService},
    models::response::ApiResponse,
};
pub use sqlx::PgPool;

pub struct AttendancePoliciesService {}

impl IService<AttendancePolicies> for AttendancePoliciesService {
    async fn get_all(connection: PgPool) -> ApiResponse<Vec<AttendancePolicies>> {
        match AttendancePoliciesRepository::get_all(connection).await {
            Ok(entity) => ApiResponse::success(entity),
            Err(e) => ApiResponse::error(e.to_string()),
        }
    }

    async fn get_by_filter(connection: PgPool, filter: &String) -> ApiResponse<Vec<AttendancePolicies>> {
        match AttendancePoliciesRepository::get_by_filter(connection, filter).await {
            Ok(entity) => ApiResponse::success(entity),
            Err(e) => ApiResponse::error(e.to_string()),
        }
    }

    async fn add(connection: PgPool, entity: &AttendancePolicies) -> ApiResponse<bool> {
        match AttendancePoliciesRepository::add(connection, entity).await {
            Ok(entity) => ApiResponse::success(entity),
            Err(e) => ApiResponse::error(e.to_string()),
        }
    }

    async fn update(connection: PgPool, entity: &AttendancePolicies) -> ApiResponse<bool> {
        match AttendancePoliciesRepository::update(connection, entity).await {
            Ok(entity) => ApiResponse::success(entity),
            Err(e) => ApiResponse::error(e.to_string()),
        }
    }

    async fn delete(connection: PgPool, id: &String) -> ApiResponse<bool> {
        match AttendancePoliciesRepository::delete(connection, id).await {
            Ok(entity) => ApiResponse::success(entity),
            Err(e) => ApiResponse::error(e.to_string()),
        }
    }
}
