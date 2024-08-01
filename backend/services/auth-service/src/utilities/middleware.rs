use std::future::{ready, Ready};

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
use jsonwebtoken::{
    decode,
    errors::{Error as JwtError, ErrorKind},
    Algorithm, DecodingKey, Validation,
};

use crate::models::{
    config::app_config::get_json, constants, response::ApiResponse, roles::auth::JwtClaims,
};

use super::redis_client::RedisClient;

pub struct InterHandler;

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
        let redis_client = match RedisClient::new() {
            Ok(client) => client,
            Err(err) => {
                return Box::pin(async move {
                    let (request, _pl) = req.into_parts();
                    let response = HttpResponse::InternalServerError()
                        .json(ApiResponse::<String>::error(format!(
                            "Internal error: {}",
                            err
                        )))
                        .map_into_right_body();
                    Ok(ServiceResponse::new(request, response))
                });
            }
        };

        let json = match get_json() {
            Ok(cfg) => cfg,
            Err(err) => {
                return Box::pin(async move {
                    let (request, _pl) = req.into_parts();
                    let response = HttpResponse::InternalServerError()
                        .json(ApiResponse::<String>::error(format!(
                            "Internal error: {}",
                            err
                        )))
                        .map_into_right_body();
                    Ok(ServiceResponse::new(request, response))
                });
            }
        };

        let mut authenticate_pass = false;

        let mut headers = req.headers().clone();
        headers.append(
            HeaderName::from_static("content-length"),
            HeaderValue::from_static("true"),
        );

        if Method::OPTIONS == *req.method() {
            authenticate_pass = true;
        } else {
            for ignore_route in constants::IGNORE_ROUTES.iter() {
                if req.path().starts_with(ignore_route) {
                    authenticate_pass = true;
                    break;
                }
            }
        }

        if !authenticate_pass {
            if let Some(authen_header) = req.headers().get(constants::AUTHORIZATION) {
                if let Ok(authen_str) = authen_header.to_str() {
                    if authen_str.starts_with("bearer") || authen_str.starts_with("Bearer") {
                        let token = authen_str.replace("Bearer ", "");
                        let claim = match verify_token(
                            &token,
                            &json.token_provider.token_security_key,
                            &json.token_provider.token_audience,
                        ) {
                            Ok(claim) => claim,
                            Err(_) => {
                                let (request, _pl) = req.into_parts();
                                let response = HttpResponse::Unauthorized()
                                    .json(ApiResponse::<String>::error(
                                        constants::MESSAGE_INVALID_TOKEN.to_owned(),
                                    ))
                                    .map_into_right_body();
                                return Box::pin(async {
                                    Ok(ServiceResponse::new(request, response))
                                });
                            }
                        };

                        match redis_client.get_key(&claim.sid) {
                            Ok(store_token) if token == store_token => {
                                let path = req.path();
                                let transformed_path =
                                    path.strip_prefix("/api/").unwrap_or(path).replace("/", "-");

                                let allowed = claim
                                    .role
                                    .iter()
                                    .any(|role| transformed_path.contains(&role.route));

                                if allowed {
                                    authenticate_pass = true;
                                }
                            }
                            Ok(_) => {
                                authenticate_pass = false;
                            }
                            Err(err) => {
                                return Box::pin(async move {
                                    let (request, _pl) = req.into_parts();
                                    let response = HttpResponse::InternalServerError()
                                        .json(ApiResponse::<String>::error(format!(
                                            "Internal error: {}",
                                            err
                                        )))
                                        .map_into_right_body();
                                    Ok(ServiceResponse::new(request, response))
                                });
                            }
                        }
                    }
                }
            }
        }

        if !authenticate_pass {
            let (request, _pl) = req.into_parts();
            let response = HttpResponse::Unauthorized()
                .json(ApiResponse::<String>::error(
                    constants::MESSAGE_INVALID_TOKEN.to_owned(),
                ))
                .map_into_right_body();
            return Box::pin(async { Ok(ServiceResponse::new(request, response)) });
        }

        let res = self.service.call(req);

        Box::pin(async move { res.await.map(ServiceResponse::map_into_left_body) })
    }
}

pub fn verify_token(token: &str, secret: &str, audience: &str) -> Result<JwtClaims, JwtError> {
    let decoding_key = DecodingKey::from_secret(secret.as_bytes());
    let mut validation = Validation::new(Algorithm::HS256);

    // Set the expected audience
    validation.set_audience(&[audience]);

    // Decode and validate the token
    match decode::<JwtClaims>(token, &decoding_key, &validation) {
        Ok(token_data) => {
            let now = jsonwebtoken::get_current_timestamp();
            if token_data.claims.exp < now {
                Err(ErrorKind::ExpiredSignature.into())
            } else {
                Ok(token_data.claims)
            }
        }
        Err(err) => Err(err),
    }
}
