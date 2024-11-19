use crate::models::{config::app_config::AppConfig, response::ApiResponse};
use actix_web::{http::Method, HttpResponse};
use awc::{body::to_bytes, Client};
use serde::{Deserialize, Serialize};

pub struct ServiceCommunicator {
    config: AppConfig,
}

impl ServiceCommunicator {
    pub fn new(config: AppConfig) -> Self {
        ServiceCommunicator { config }
    }

    pub async fn call_service<T: Serialize + for<'de> Deserialize<'de>>(
        &self,
        path_and_query: &str,
        method: Method,
        payload: Option<T>,
    ) -> HttpResponse {
        let client = Client::new();

        // Find the matching service from the config
        let target_url = self
            .config
            .services
            .iter()
            .find(|service| path_and_query.starts_with(&format!("/{}", service.name)))
            .map(|service| format!("http://{}:{}{}", service.host, service.port, path_and_query));

        if let Some(url) = target_url {
            let request = client
                .request(method.clone(), url)
                .append_header(("User-Agent", "Actix-web"));

            // Send the request with the payload if provided
            let response = match method {
                Method::POST => {
                    if let Some(data) = payload {
                        request.send_json(&data).await
                    } else {
                        request.send().await
                    }
                }
                Method::GET => request.send().await,
                _ => {
                    return HttpResponse::InternalServerError()
                        .body("Unsupported HTTP method".to_string());
                }
            };

            match response {
                Ok(mut res) => {
                    // Read the response body
                    let body = res.body().await;

                    match body {
                        Ok(api_response) => HttpResponse::Ok().body(api_response),
                        Err(_) => HttpResponse::InternalServerError()
                            .body("Failed to parse response".to_string()),
                    }
                }
                Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
            }
        } else {
            HttpResponse::InternalServerError().body("Invalid API section".to_string())
        }
    }

    pub async fn get_data<T: Serialize + for<'de> Deserialize<'de>>(
        &self,
        response: HttpResponse,
    ) -> Result<ApiResponse<T>, actix_web::Error> {
        // Convert the response body to bytes
        let body_bytes = to_bytes(response.into_body()).await?;

        // Deserialize the body into ApiResponse<T>
        let api_response: ApiResponse<T> = serde_json::from_slice(&body_bytes)
            .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;

        // Return the ApiResponse<T> directly
        Ok(api_response)
    }
}
