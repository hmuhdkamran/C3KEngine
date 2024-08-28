use sk3_code_generator::{
    config::configuration::get_json, 
    handler::{file_handler::FileHandler, postgresql_handler::PostgreSqlHandler}};
pub use sqlx::{
    pool::PoolConnection,
    postgres::{PgArguments, PgPoolOptions, PgRow},
    Arguments, PgPool, Postgres, Row,
};
use std::error::Error as StdError;

#[tokio::main]
async fn main() -> Result<(), Box<dyn StdError>> {
    let config = match get_json() {
        Ok(config) => config,
        Err(e) => {
            eprintln!("Error loading configuration: {}", e);
            return Err(e);
        }
    };

    let schemas = PostgreSqlHandler::get_schemas(&config.connection_string).await?;

    let file_handler = FileHandler::new(schemas, config);

    file_handler.process_files()?;

    Ok(())
}
