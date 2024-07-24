use futures::Future;
use sqlx::{postgres::PgRow, PgPool};
use std::error::Error as StdError;

pub trait Model: Sized {
    fn from_row(row: &PgRow) -> Self;
}

pub trait IRepository<TEntity>
where
    TEntity: Model,
{
    /// get all entities
    fn get_all(
        connection: PgPool,
    ) -> impl Future<Output = Result<Vec<TEntity>, Box<dyn StdError>>> + Send;

    /// get a entity by filter
    fn get_by_filter(
        connection: PgPool,
        filter: &String,
    ) -> impl Future<Output = Result<Vec<TEntity>, Box<dyn StdError>>> + Send;

    /// add an entity to the database
    fn add(
        connection: PgPool,
        entity: &TEntity,
    ) -> impl Future<Output = Result<bool, Box<dyn StdError>>> + Send;

    /// update an entity
    fn update(
        connection: PgPool,
        entity: &TEntity,
    ) -> impl Future<Output = Result<bool, Box<dyn StdError>>> + Send;

    /// delete an entity by its id
    fn delete(
        connection: PgPool,
        id: &String,
    ) -> impl Future<Output = Result<bool, Box<dyn StdError>>> + Send;
}
