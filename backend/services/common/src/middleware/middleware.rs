use actix_web::web::Data;
use std::collections::HashSet;
use std::future::{ready, Ready};
use std::sync::Arc;
use uuid::Uuid;

use actix_web::{
    body::EitherBody,
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    http::{
        header::{HeaderName, HeaderValue},
        Method,
    },
    Error, HttpResponse,
};

use futures::future::LocalBoxFuture;

use crate::models::config::app_config::AppConfig;
use crate::{
    handler::redis_handler::RedisHandler,
    models::{auth::Auth, constants, response::ApiResponse},
    utilities::security_utils::SecurityUtils,
};

pub struct InterHandler;

fn extract_path_data(path: &str, claim_sid: &str) -> (String, String) {
    let clean_path = path.strip_prefix("/api/").unwrap_or(path);
    let mut parts = clean_path.split('/');

    let first_part = parts.next().unwrap_or_default();
    let role = format!("{}-api/{}", claim_sid, first_part);

    let claims = parts
        .filter(|part| Uuid::parse_str(part).is_err())
        .filter(|part| !part.contains(&['"', '='][..]))
        .collect::<Vec<_>>()
        .join("-");

    (role, claims)
}

impl<S, B> Transform<S, ServiceRequest> for InterHandler
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type InitError = ();
    type Transform = InterHandlerMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(InterHandlerMiddleware { service }))
    }
}

pub struct InterHandlerMiddleware<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for InterHandlerMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        // Precompute frequently used values
        let path = req.path().to_string();
        let method = req.method().clone();
        let headers = req.headers().clone();

        // Handle service initialization errors first
        let redis_client = req
            .app_data::<Data<Arc<RedisHandler>>>()
            .expect("RedisHandler missing")
            .get_ref();

        let config = req
            .app_data::<Data<Arc<AppConfig>>>()
            .expect("Config missing")
            .get_ref();

        // Early exit for OPTIONS or ignored routes
        let ignore_routes: HashSet<&'static str> =
            constants::IGNORE_ROUTES.iter().copied().collect();
        if method == Method::OPTIONS || ignore_routes.contains(path.as_str()) {
            return process_request(self.service.call(req));
        }

        // Process authorization header
        let auth_header = headers
            .get(constants::AUTHORIZATION)
            .and_then(|v| v.to_str().ok());

        let token = match auth_header {
            Some(h) if h.to_lowercase().starts_with("bearer ") => h.split_whitespace().nth(1),
            _ => return unauthorized(req),
        };

        let token = match token {
            Some(t) => t,
            None => return unauthorized(req),
        };

        // Verify token
        let claim = match SecurityUtils::verify_token(
            token,
            &config.token_provider.token_security_key,
            &config.token_provider.token_audience,
            &config.token_provider.token_security_algorithm,
        ) {
            Ok(c) => c,
            Err(_) => return unauthorized(req),
        };

        // Redis checks
        let stored_token = match redis_client.get_key(&claim.sid) {
            Ok(t) => t,
            Err(e) => return error_response(req, format!("Redis error: {}", e)),
        };

        if stored_token != token {
            return unauthorized(req);
        }

        // Path processing and authorization check
        let (api_app, api_claim) = extract_path_data(&path, &claim.sid);
        let app_keys: Vec<Auth> = match redis_client
            .get_key(&api_app)
            .and_then(|s| serde_json::from_str(&s).map_err(|e| e.into()))
        {
            Ok(k) => k,
            Err(e) => return error_response(req, format!("Authorization error: {}", e)),
        };

        if !app_keys.iter().any(|k| k.route == api_claim) {
            return unauthorized(req);
        }

        // Add security headers
        let mut req = req;
        req.headers_mut().insert(
            HeaderName::from_static("content-length"),
            HeaderValue::from_static("true"),
        );

        process_request(self.service.call(req))
    }
}

// Helper functions
fn unauthorized<B: 'static>(
    req: ServiceRequest,
) -> LocalBoxFuture<'static, Result<ServiceResponse<EitherBody<B>>, Error>> {
    let (request, _) = req.into_parts();
    let response = HttpResponse::Unauthorized()
        .json(ApiResponse::<String>::error(
            constants::MESSAGE_INVALID_TOKEN.to_string(),
        ))
        .map_into_right_body();
    Box::pin(async { Ok(ServiceResponse::new(request, response)) })
}

fn error_response<B: 'static>(
    req: ServiceRequest,
    msg: String,
) -> LocalBoxFuture<'static, Result<ServiceResponse<EitherBody<B>>, Error>> {
    let (request, _) = req.into_parts();
    let response = HttpResponse::InternalServerError()
        .json(ApiResponse::<String>::error(msg))
        .map_into_right_body();
    Box::pin(async { Ok(ServiceResponse::new(request, response)) })
}

fn process_request<B>(
    fut: impl std::future::Future<Output = Result<ServiceResponse<B>, Error>> + 'static,
) -> LocalBoxFuture<'static, Result<ServiceResponse<EitherBody<B>>, Error>> {
    Box::pin(async move { fut.await.map(|res| res.map_into_left_body()) })
}
