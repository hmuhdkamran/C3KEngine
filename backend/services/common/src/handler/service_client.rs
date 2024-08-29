use actix_web::http::Method;
use serde::{Deserialize, Serialize};
use awc::Client;
use std::sync::Arc;

pub struct ServiceCommunicator {
    config: Arc<AppConfig>,
}

impl ServiceCommunicator {
    pub fn new(config: Arc<AppConfig>) -> Self {
        ServiceCommunicator { config }
    }

    pub async fn call_service<T: Serialize + for<'de> Deserialize<'de>>(
        &self,
        path_and_query: &str,
        method: Method,
        payload: Option<T>,
    ) -> ApiResponse<T> {
        let client = Client::new();

        // Find the matching service from the config
        let target_url = self
            .config
            .services
            .iter()
            .find(|service| path_and_query.starts_with(&format!("/{}", service.name)))
            .map(|service| format!("http://{}:{}{}", service.host, service.port, path_and_query));

        if let Some(url) = target_url {
            let mut request = client.request(method.clone(), url);

            // Set the appropriate headers
            request = request.append_header(("User-Agent", "Actix-web"));

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
                    return ApiResponse::error("Unsupported HTTP method".to_string());
                }
            };

            match response {
                Ok(mut res) => {
                    // Read the response body
                    let body = res.json::<ApiResponse<T>>().await;

                    match body {
                        Ok(api_response) => api_response,
                        Err(_) => ApiResponse::error("Failed to parse response".to_string()),
                    }
                }
                Err(err) => ApiResponse::error(err.to_string()),
            }
        } else {
            ApiResponse::error("Invalid API section".to_string())
        }
    }
}
