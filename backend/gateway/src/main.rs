use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use actix_web::dev::Server;
use actix_web::http::header;
use actix_web::middleware::Logger;
use actix_web::web::Json;
use actix_web::web::Data;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use std::time::Duration;
use log::info;

const SECRET_KEY: &'static str = "secret_key"; // Use a stronger secret in a real-world scenario

#[derive(Debug, Serialize, Deserialize)]
struct ServiceConfig {
    name: String,
    address: String,
}

struct ServiceRegistry {
    services: Arc<RwLock<HashMap<String, String>>>,  // Service Name -> Service Address (URL/URI)
}

impl ServiceRegistry {
    fn new() -> Self {
        ServiceRegistry {
            services: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    fn register(&self, name: String, address: String) {
        let mut services = self.services.write().unwrap();
        services.insert(name, address);
    }

    fn deregister(&self, name: &str) {
        let mut services = self.services.write().unwrap();
        services.remove(name);
    }

    fn get_address(&self, name: &str) -> Option<String> {
        let services = self.services.read().unwrap();
        services.get(name).cloned()
    }
}

#[derive(Debug, Deserialize)]
struct RegisterServiceRequest {
    name: String,
    address: String,
}

async fn register_service(
    registry: Data<ServiceRegistry>,
    req: Json<RegisterServiceRequest>,
) -> impl Responder {
    registry.register(req.name.clone(), req.address.clone());
    HttpResponse::Ok().body("Service registered successfully")
}

#[derive(Debug, Deserialize)]
struct DeregisterServiceRequest {
    name: String,
}

async fn deregister_service(
    registry: Data<ServiceRegistry>,
    req: Json<DeregisterServiceRequest>,
) -> impl Responder {
    registry.deregister(&req.name);
    HttpResponse::Ok().body("Service deregistered successfully")
}

struct RateLimiter {
    visitors: Arc<Mutex<HashMap<SocketAddr, u32>>>,
}

impl RateLimiter {
    fn new() -> Self {
        RateLimiter {
            visitors: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    fn allow(&self, addr: SocketAddr) -> bool {
        let mut visitors = self.visitors.lock().unwrap();
        let counter = visitors.entry(addr).or_insert(0);
        if *counter >= 5 {  // Allow up to 5 requests
            false
        } else {
            *counter += 1;
            true
        }
    }
}

fn authenticate(token: &str) -> bool {
    // let validation = Validation {
    //     iss: Some("my_issuer".to_string()),
    //     algorithms: vec![Algorithm::HS256],
    //     ..Default::default()
    // };

    // match decode::<serde_json::Value>(&token, &DecodingKey::from_secret(SECRET_KEY.as_ref()), &validation) {
    //     Ok(_data) => true,
    //     Err(err) => {
    //         eprintln!("JWT Decoding error: {:?}", err);
    //         match *err.kind() {
    //             ErrorKind::InvalidToken => false,  // token is invalid
    //             _ => false
    //         }
    //     }
    // }

    return true;
}

async fn service_handler(
    req: HttpRequest,
    body: web::Bytes,
    registry: Data<ServiceRegistry>,
    client: Data<awc::Client>,
) -> impl Responder {
    let path = req.uri().path();
    let parts: Vec<&str> = path.split('/').collect();

    if parts.len() < 2 {
        return HttpResponse::BadRequest().body("Invalid request URI");
    }

    let service_name = parts[1];

    match registry.get_address(service_name) {
        Some(address) => {
            let forward_uri = format!("{}{}", address, req.uri());

            let mut client_req = client.request_from(forward_uri, req.head());

            // Add custom header
            client_req = client_req.header("X-Custom-Header", "My API Gateway");

            // Forward request and handle response
            match client_req.send_body(body).await {
                Ok(mut res) => {
                    let mut body_bytes = res.body().await.unwrap_or_default().to_vec();
                    let mut data: serde_json::Value = serde_json::from_slice(&body_bytes).unwrap_or_else(|_| json!({}));
                    data["custom"] = json!("This data is added by the gateway");

                    HttpResponse::build(res.status()).body(data.to_string())
                }
                Err(_) => HttpResponse::BadGateway().body("Failed to forward request"),
            }
        }
        None => HttpResponse::NotFound().body("Service not found"),
    }
}

async fn router(
    req: HttpRequest,
    body: web::Bytes,
    registry: Data<ServiceRegistry>,
    client: Data<awc::Client>,
) -> impl Responder {
    let path = req.uri().path();

    if path == "/register_service" {
        return register_service(registry, Json::from_slice(&body).unwrap()).await;
    }

    if path == "/deregister_service" {
        return deregister_service(registry, Json::from_slice(&body).unwrap()).await;
    }

    // Authentication
    if let Some(auth_header) = req.headers().get(header::AUTHORIZATION) {
        let token_str = auth_header.to_str().unwrap_or("");
        if !authenticate(token_str) {
            return HttpResponse::Unauthorized().body("Unauthorized");
        }
    } else {
        return HttpResponse::Unauthorized().body("Unauthorized");
    }

    // Handle other requests using the service handler
    service_handler(req, body, registry, client).await
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();

    let registry = Data::new(ServiceRegistry::new());
    let client = Data::new(awc::Client::default());

    HttpServer::new(move || {
        App::new()
            .app_data(registry.clone())
            .app_data(client.clone())
            .wrap(Logger::default())
            .wrap(RateLimiter::default()) // Default rate limiter
            .service(
                web::resource("/*")
                    .route(web::to(router))
            )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
