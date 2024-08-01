use actix_multipart::{Field, MultipartError};
use actix_web::http::header::ContentDisposition;
use actix_web::{error, middleware, web, App, Error, HttpServer};
use bytes::Bytes;
use env_logger;
use futures::{Future, Stream};
use reqwest;
use reqwest::Url;
use reqwest::{Client, Response};
use std::sync::Arc;

static LISTEN_AT: String = "0.0.0.0:80".to_string();

fn main() -> std::io::Result<()> {
    let address: std::net::SocketAddrV4 = LISTEN_AT.parse().unwrap();
    //TODO: Custom http client
    let client_builder = Client::builder();
    // client_builder.cookie_store(true);
    // client_builder.use_rustls_tls();
    let http_client = Arc::new(client_builder.build().unwrap());
    env_logger::init();

    // Start http server
    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .data(http_client.clone())
    })
    .bind(address)?
    .run()
}