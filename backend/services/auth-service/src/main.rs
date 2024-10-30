use actix_cors::Cors;
use actix_web::{web, App, HttpServer};
use c3k_auth_service::controllers::roles::{
    auth_controller::auth_routes, role_controller::role_routes,
    role_route_map_controller::role_route_map_routes, route_controller::route_routes,
    user_controller::user_routes, user_role_map_controller::user_role_map_routes,
};
use c3k_common::{
    handler::service_client::ServiceCommunicator, models::config::app_config::get_json,
};
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

    let service = config
        .services
        .iter()
        .find(|f| f.name == "api/auth")
        .unwrap();

    let addr = format!("{}:{}", service.host, service.port);
    let db_pool = create_db_pool(&service.connection_string)
        .await
        .expect("Failed to create pool");

    let server = HttpServer::new(move || {
        let mut cors = Cors::default()
            .allow_any_method()
            .allow_any_header()
            .supports_credentials();

        cors = cors.allowed_origin(
            format!("http://{}:{}", config.gateway.host, config.gateway.port).as_str(),
        );

        for origin in &config.services {
            cors = cors.allowed_origin(format!("http://{}:{}", origin.host, origin.port).as_str());
        }

        let communicator = ServiceCommunicator::new(config.clone());

        App::new()
            .wrap(cors)
            .app_data(web::Data::new(db_pool.clone()))
            .app_data(web::Data::new(communicator))
            .configure(user_routes)            
            .configure(role_routes)
            .configure(route_routes)
            .configure(user_role_map_routes)
            .configure(role_route_map_routes)
            .configure(auth_routes)
    });

    println!("App is Running on http://{}", addr);
    server.bind(addr)?.run().await
}
