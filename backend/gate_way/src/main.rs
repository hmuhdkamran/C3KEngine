use actix_cors::Cors;
use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer};
use awc::Client;
use c3k_common::{
    middleware::middleware::InterHandler,
    models::config::app_config::{get_config, initialize_config, AppConfig},
};
use std::{
    io::{Error, ErrorKind},
    sync::Arc,
};

#[actix_web::main]
async fn main() -> Result<(), std::io::Error> {
    if let Err(err) = initialize_config().await {
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
            .wrap(cors)
            .wrap(InterHandler)
            .app_data(web::Data::new(Arc::new(config.clone())))
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

    // Find the matching service from the config
    let target_url = config
        .services
        .iter()
        .find(|service| path_and_query.starts_with(&format!("/{}", service.name)))
        .map(|service| format!("http://{}:{}{}", service.host, service.port, path_and_query));

    // Check if a valid target URL was found
    if let Some(url) = target_url {
        // Forward the request to the appropriate service
        let mut forward_request = client
            .request(req.method().clone(), url)
            .append_header(("User-Agent", "Actix-web"));

        // Copy original request headers to the forwarded request
        for (key, value) in req.headers().iter() {
            forward_request = forward_request.append_header((key.clone(), value.clone()));
        }

        // Send the request with the original payload
        let response = forward_request.send_stream(payload).await;

        match response {
            Ok(mut res) => {
                // Extract the content type before moving the mutable borrow
                let content_type = res
                    .headers()
                    .get("Content-Type")
                    .and_then(|ct| ct.to_str().ok())
                    .unwrap_or("application/json")
                    .to_string();

                // Read the response body
                let body = res.body().await;
                match body {
                    Ok(bytes) => HttpResponse::build(res.status())
                        .content_type(content_type)
                        .body(bytes),
                    Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
                }
            }
            Err(err) => match err {
                awc::error::SendRequestError::Timeout => {
                    HttpResponse::GatewayTimeout().body("Request timed out")
                }
                awc::error::SendRequestError::Connect(_) => {
                    HttpResponse::BadGateway().body("Could not connect to target service")
                }
                _ => HttpResponse::InternalServerError().body(err.to_string()),
            },
        }
    } else {
        // Return a bad request response if no valid target URL is found
        HttpResponse::BadRequest().body("Invalid API section")
    }
}
