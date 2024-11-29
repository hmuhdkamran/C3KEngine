use actix_cors::Cors;
use actix_web::{web, App, HttpServer};
use c3k_auth_service::controllers::{
    role::{
        auth_controller::auth_routes, products::products_routes,
        role_route_maps::role_route_maps_routes, roles::roles_routes, routes::routes_routes,
        user_product_maps::user_product_maps_routes, user_role_maps::user_role_maps_routes,
        users::users_routes,
    },
    setup::{
        areas::areas_routes, brand_social_maps::brand_social_maps_routes, brands::brands_routes,
        building_brand_maps::building_brand_maps_routes, building_spaces::building_spaces_routes,
        buildings::buildings_routes, business::business_routes, cities::cities_routes,
        countries::countries_routes, floors::floors_routes,
        ownership_status::ownership_status_routes, socials::socials_routes,
        space_types::space_types_routes, status::status_routes,
    },
};
use c3k_common::{
    handler::{
        service_client::ServiceCommunicator, 
        redis_handler::RedisHandler
    }, models::config::app_config::{create_db_pool, initialize_config, get_config},
};
pub use sqlx::{
    pool::PoolConnection,
    postgres::{PgArguments, PgPoolOptions, PgRow},
    Arguments, PgPool, Postgres, Row,
};
use std::{io::{Error, ErrorKind}, sync::Arc};

#[actix_web::main]
async fn main() -> Result<(), std::io::Error> {
    if let Err(err) = initialize_config().await  {
        eprintln!("Failed to initialize configuration: {}", err);
        std::process::exit(1);
    }

    let config = match get_config() {
        Some(cfg) => cfg,
        None => {
            eprintln!("Internal error: Configuration not initialized");
            return Err(Error::new(ErrorKind::Other, "Configuration error"));
        }
    };

    let service = config
        .services
        .iter()
        .find(|f| f.name == "api/auth")
        .unwrap();

    let addr = format!("{}:{}", service.host, service.port);
    let db_pool = match create_db_pool(&service.connection_string).await {
        Ok(pool) => pool,
        Err(err) => {
            eprintln!("Failed to create DB pool: {}", err);
            return Err(Error::new(ErrorKind::Other, "Database pool initialization failed"));
        }
    };
    let redis_client = RedisHandler::new()
        .map_err(|e| Error::new(ErrorKind::Other, format!("Redis initialization failed: {}", e)))?;

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

        let communicator = ServiceCommunicator::new(Arc::new(config.clone()));

        App::new()
            .wrap(cors)
            .app_data(web::Data::new(db_pool.clone()))
            .app_data(web::Data::new(redis_client.clone()))
            .app_data(web::Data::new(communicator))             
            .configure(products_routes)
            .configure(roles_routes)
            .configure(routes_routes)
            .configure(user_role_maps_routes)
            .configure(role_route_maps_routes)
            .configure(user_product_maps_routes)
            .configure(areas_routes)
            .configure(brand_social_maps_routes)
            .configure(brands_routes)
            .configure(building_brand_maps_routes)
            .configure(building_spaces_routes)
            .configure(buildings_routes)
            .configure(business_routes)
            .configure(cities_routes)
            .configure(countries_routes)
            .configure(floors_routes)
            .configure(ownership_status_routes)
            .configure(socials_routes)
            .configure(space_types_routes)
            .configure(status_routes)
            .configure(auth_routes)
            .configure(users_routes)
    });

    println!("App is Running on http://{}", addr);
    server.bind(addr)?.run().await
}
