use futures::Future;
use sqlx::PgPool;

use crate::models::response::ApiResponse;
use super::irepository::Model;

pub trait IService<TEntity>
where
    TEntity: Model,
{
    /// get all entities
    fn get_all(connection: PgPool) -> impl Future<Output = ApiResponse<Vec<TEntity>>> + Send;

    /// get a multiple entity by filter
    fn get_by_filter(
        connection: PgPool,
        filter: &String,
    ) -> impl Future<Output = ApiResponse<Vec<TEntity>>> + Send;

    /// add an entity to the database
    fn add(connection: PgPool, entity: &TEntity) -> impl Future<Output = ApiResponse<bool>> + Send;

    /// update an entity
    fn update(
        connection: PgPool,
        entity: &TEntity,
    ) -> impl Future<Output = ApiResponse<bool>> + Send;

    /// delete an entity by its id
    fn delete(connection: PgPool, id: &String) -> impl Future<Output = ApiResponse<bool>> + Send;
}
