use actix_cors::Cors;
use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer};
use awc::Client;
use c3k_common::{
    middleware::middleware::InterHandler,
    models::config::app_config::{get_json, AppConfig},
};
use std::{
    io::{Error, ErrorKind},
    sync::Arc,
};

#[actix_web::main]
async fn main() -> Result<(), std::io::Error> {
    let config = match get_json() {
        Ok(cfg) => Arc::new(cfg),
        Err(err) => {
            eprintln!("Error loading configuration: {}", err);
            return Err(Error::new(ErrorKind::Other, "Configuration error"));
        }
    };

    let addr = format!("{}:{}", config.gateway.host, config.gateway.port);

    let server = HttpServer::new(move || {
        let mut cors = Cors::default()
            .allow_any_method()
            .allow_any_header()
            .supports_credentials();

        for origin in &config.gateway.cors {
            cors = cors.allowed_origin(origin);
        }

        App::new()
            .app_data(web::Data::new(config.clone()))
            .wrap(cors)
            .wrap(InterHandler)
            .default_service(web::route().to(forward_request))
    });

    println!("App is Running on http://{}", addr);
    server.bind(addr)?.run().await?;

    Ok(())
}

async fn forward_request(
    req: HttpRequest,
    payload: web::Payload,
    config: web::Data<Arc<AppConfig>>,
) -> HttpResponse {
    let client = Client::new();

    // Get the full path including the query string
    let path_and_query = req
        .uri()
        .path_and_query()
        .map(|pq| pq.as_str())
        .unwrap_or("");

    // Get the path after /api/
    let target_url = config
        .services
        .iter()
        .find(|service| path_and_query.starts_with(&format!("/{}", service.name)))
        .map(|service| format!("http://{}:{}{}", service.host, service.port, path_and_query))
        .unwrap_or_else(|| return format!("Invalid API section"));

    // Forward the request to the appropriate service
    let mut forward_request = client
        .request(req.method().clone(), target_url)
        .append_header(("User-Agent", "Actix-web"));

    for (key, value) in req.headers().iter() {
        forward_request = forward_request.append_header((key.clone(), value.clone()));
    }

    // Send the request with the original payload
    let response = forward_request.send_stream(payload).await;

    match response {
        Ok(mut res) => {
            let body = res.body().await;
            match body {
                Ok(bytes) => HttpResponse::build(res.status()).body(bytes),
                Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
            }
        }
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}
