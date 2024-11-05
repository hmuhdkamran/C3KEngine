use futures::Future;
use sqlx::{postgres::PgRow, PgPool};
use std::error::Error as StdError;

pub trait Model: Sized {
    fn from_row(row: &PgRow) -> Self;

    fn select_string(columns: &[&'static str]) -> String {
        let mut columns_str = String::new();
        let mut first = true;

        for &column in columns {
            if !first {
                columns_str.push_str(", ");
            }
            columns_str.push_str(&format!("\"{}\"", column));
            first = false;
        }
        columns_str
    }

    fn build_update_string(table_name: &str, columns: &[&'static str], primary_key: &str) -> String {
        let mut update_str = format!("UPDATE {}", table_name);
        let mut first = true;

        for (i, &column) in columns.iter().enumerate() {
            if !first {
                update_str.push_str(", ");
            }
            update_str.push_str(&format!(r#""{}"=${}"#, column, i + 2));
            first = false;
        }

        update_str.push_str(&format!(" WHERE \"{}\"=$1", primary_key));

        update_str
    }

    fn build_select_string(table_name: &str, columns: &[&'static str], where_filter: Option<&str>) -> String {
        let columns_str = Self::select_string(columns);
        let mut query = format!("SELECT {} FROM {}", columns_str, table_name);

        if let Some(filter) = where_filter {
            query.push_str(&format!(" WHERE {}", filter));
        }

        query
    }

    fn build_insert_string(table_name: &str, columns: &[&'static str]) -> String {
        let columns_str = Self::select_string(columns);
        let placeholders: Vec<String> = (1..=columns.len()).map(|i| format!("${}", i)).collect();
        let values_str = placeholders.join(", ");
        format!(
            "INSERT INTO {} ({}) VALUES ({})",
            table_name, columns_str, values_str
        )
    }

    fn build_delete_string(table_name: &str, primary_key: &str) -> String {
        format!("DELETE FROM {} WHERE \"{}\"::TEXT=$1", table_name, primary_key)
    }
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
