use actix_cors::Cors;
use actix_web::{middleware::Logger, web, App, HttpServer};
use c3k_backend::{
    controllers::roles::{
        auth_controller::auth_routes, role_controller::role_routes,
        role_route_map_controller::role_route_map_routes, route_controller::route_routes,
        user_controller::user_routes, user_role_map_controller::user_role_map_routes,
    },
    utilities::middleware::InterHandler,
};
use c3k_common::models::config::app_config::get_json;
pub use sqlx::{
    pool::PoolConnection,
    postgres::{PgArguments, PgPoolOptions, PgRow},
    Arguments, PgPool, Postgres, Row,
};

use std::io::{Error, ErrorKind};
use std::sync::Arc;

async fn create_db_pool(connection_string: &str) -> Result<PgPool, sqlx::Error> {
    PgPoolOptions::new()
        .max_connections(5)
        .connect(connection_string)
        .await
        .map_err(|e| {
            eprintln!("Failed to create database pool: {}", e);
            e
        })
}

#[actix_web::main]
async fn main() -> Result<(), std::io::Error> {
    let config = match get_json() {
        Ok(cfg) => Arc::new(cfg),
        Err(err) => {
            eprintln!("Error loading configuration: {}", err);
            return Err(Error::new(ErrorKind::Other, "Configuration error"));
        }
    };

    let addr = format!(
        "{}:{}",
        config.backend_engine.host, config.backend_engine.port
    );
    let db_pool = create_db_pool(&config.data.connection_string)
        .await
        .expect("Failed to create pool");

    let server = HttpServer::new(move || {
        let mut cors = Cors::default()
            .allow_any_method()
            .allow_any_header()
            .supports_credentials();

        for origin in &config.backend_engine.cors {
            cors = cors.allowed_origin(origin);
        }

        App::new()
            .wrap(cors)
            .wrap(Logger::default())
            .wrap(InterHandler)
            .app_data(web::Data::new(db_pool.clone()))
            .configure(auth_routes)
            .configure(user_routes)
            .configure(role_routes)
            .configure(route_routes)
            .configure(user_role_map_routes)
            .configure(role_route_map_routes)
    });

    println!("App is Running on http://{}", addr);
    server.bind(addr)?.run().await
}
