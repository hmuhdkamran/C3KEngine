use actix_cors::Cors;
use actix_web::{middleware::Logger, web, App, HttpServer};
use c3k_common::{handler::service_client::ServiceCommunicator, models::config::app_config::get_json};
use c3k_retail_service::controllers::{
    customer::customer_contacts::customer_contacts_routes,
    customer::customer_loyalty_points::customer_loyalty_points_routes,
    customer::customer_packages::customer_packages_routes,
    customer::customer_types::customer_types_routes,
    customer::customers::customers_routes,
    customer::customers_addresses::customers_addresses_routes,
    customer::package_rules::package_rules_routes,
    customer::package_types::package_types_routes,
    customer::packages::packages_routes,
    inventory::attributes::attributes_routes,
    inventory::brands::brands_routes,
    inventory::categories::categories_routes,
    inventory::product_attributes::product_attributes_routes,
    inventory::product_categories::product_categories_routes,
    inventory::product_prices::product_prices_routes,
    inventory::product_type::product_type_routes,
    inventory::product_warehouses::product_warehouses_routes,
    inventory::products::products_routes,
    inventory::units::units_routes,
    inventory::vendor_addresses::vendor_addresses_routes,
    inventory::vendor_contacts::vendor_contacts_routes,
    inventory::vendors::vendors_routes,
    point_of_sale::campaign_products::campaign_products_routes,
    point_of_sale::campaigns::campaigns_routes,
    point_of_sale::discount_types::discount_types_routes,
    point_of_sale::grn_order_details::grn_order_details_routes,
    point_of_sale::grn_orders::grn_orders_routes,
    point_of_sale::order_statuses::order_statuses_routes,
    point_of_sale::order_types::order_types_routes,
    point_of_sale::payment_methods::payment_methods_routes,
    point_of_sale::payment_terms::payment_terms_routes,
    point_of_sale::sale_order_customer_points::sale_order_customer_points_routes,
    point_of_sale::sale_order_details::sale_order_details_routes,
    point_of_sale::sale_order_payments::sale_order_payments_routes,
    point_of_sale::sale_orders::sale_orders_routes,
    point_of_sale::store_daily_expenses::store_daily_expenses_routes,
    setup::address_types::address_types_routes,
    setup::branches::branches_routes,
    setup::cities::cities_routes,
    setup::contact_types::contact_types_routes,
    setup::countries::countries_routes,
    setup::currencies::currencies_routes,
    setup::expense_types::expense_types_routes,
    setup::locations::locations_routes,
    setup::states::states_routes,
    setup::statuses::statuses_routes,
    setup::tax_rates::tax_rates_routes,
    setup::warehouses::warehouses_routes,
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

    let service = config.services.iter().find(|f| f.name == "api/hrms").unwrap();

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
            .wrap(Logger::default())
            .app_data(web::Data::new(db_pool.clone()))
            .app_data(web::Data::new(communicator))
            .configure(customer_contacts_routes)
            .configure(customer_loyalty_points_routes)
            .configure(customer_packages_routes)
            .configure(customer_types_routes)
            .configure(customers_routes)
            .configure(customers_addresses_routes)
            .configure(package_rules_routes)
            .configure(package_types_routes)
            .configure(packages_routes)
            .configure(attributes_routes)
            .configure(brands_routes)
            .configure(categories_routes)
            .configure(product_attributes_routes)
            .configure(product_categories_routes)
            .configure(product_prices_routes)
            .configure(product_type_routes)
            .configure(product_warehouses_routes)
            .configure(products_routes)
            .configure(units_routes)
            .configure(vendor_addresses_routes)
            .configure(vendor_contacts_routes)
            .configure(vendors_routes)
            .configure(campaign_products_routes)
            .configure(campaigns_routes)
            .configure(discount_types_routes)
            .configure(grn_order_details_routes)
            .configure(grn_orders_routes)
            .configure(order_statuses_routes)
            .configure(order_types_routes)
            .configure(payment_methods_routes)
            .configure(payment_terms_routes)
            .configure(sale_order_customer_points_routes)
            .configure(sale_order_details_routes)
            .configure(sale_order_payments_routes)
            .configure(sale_orders_routes)
            .configure(store_daily_expenses_routes)
            .configure(address_types_routes)
            .configure(branches_routes)
            .configure(cities_routes)
            .configure(contact_types_routes)
            .configure(countries_routes)
            .configure(currencies_routes)
            .configure(expense_types_routes)
            .configure(locations_routes)
            .configure(states_routes)
            .configure(statuses_routes)
            .configure(tax_rates_routes)
            .configure(warehouses_routes)
    });

    println!("App is Running on http://{}", addr);
    server.bind(addr)?.run().await
}
