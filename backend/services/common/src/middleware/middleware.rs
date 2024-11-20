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

use crate::{
    handler::redis_handler::RedisHandler,
    models::{auth::Auth, config::app_config::get_config, constants, response::ApiResponse},
    utilities::security_utils::SecurityUtils,
};

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
        let redis_client = match RedisHandler::new() {
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

        let json = match get_config() {
            Some(cfg) => cfg,
            None => {
                return Box::pin(async move {
                    let (request, _pl) = req.into_parts();
                    let response = HttpResponse::InternalServerError()
                        .json(ApiResponse::<String>::error(
                            "Internal error: Configuration not initialized".to_string(),
                        ))
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
                if req.path() == *ignore_route {
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
                        let claim = match SecurityUtils::verify_token(
                            &token,
                            &json.token_provider.token_security_key,
                            &json.token_provider.token_audience,
                            &json.token_provider.token_security_algorithm
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
                                let api_claim = path
                                    .strip_prefix("/api/")
                                    .unwrap_or(path)
                                    .split('/')
                                    .skip(1)
                                    .collect::<Vec<&str>>()
                                    .join("-");

                                let api_application = format!(
                                    "{}-api/{}",
                                    &claim.sid,
                                    path.split('/').nth(2).unwrap_or("")
                                );

                                match redis_client.get_key(&api_application) {
                                    Ok(application_keys_str) => {
                                        let application_keys: Vec<Auth> =
                                            match serde_json::from_str(&application_keys_str) {
                                                Ok(keys) => keys,
                                                Err(err) => {
                                                    return Box::pin(async move {
                                                        let (request, _pl) = req.into_parts();
                                                        let response =
                                                            HttpResponse::InternalServerError()
                                                                .json(ApiResponse::<String>::error(
                                                                    format!(
                                                                        "Internal error: {}",
                                                                        err
                                                                    ),
                                                                ))
                                                                .map_into_right_body();
                                                        Ok(ServiceResponse::new(request, response))
                                                    });
                                                }
                                            };

                                        authenticate_pass = application_keys
                                            .iter()
                                            .any(|key| key.route == api_claim);
                                    }
                                    Err(e) => {
                                        eprintln!(
                                            "Failed to get key {} from Redis: {:?}",
                                            api_application, e
                                        );
                                        authenticate_pass = false;
                                    }
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
