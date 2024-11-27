use actix_web::{post, web, HttpResponse, Responder};
use c3k_common::{
    handler::redis_handler::RedisHandler,
    models::auth::{AuthModel, UserProductModel},
};
use sqlx::PgPool;

use crate::services::role::{auth_service::AuthService, products::ProductsService};

#[post("/login")]
pub async fn login(
    connection: web::Data<PgPool>,
    redis_client: web::Data<RedisHandler>,
    entity: web::Json<AuthModel>,
) -> Result<impl Responder, actix_web::Error> {
    let service = AuthService::new(connection.as_ref().clone(), redis_client.as_ref().clone());
    let result = service.login(&entity.into_inner()).await;

    Ok(HttpResponse::Ok().json(result))
}

#[post("/user_products")]
pub async fn products(
    connection: web::Data<PgPool>,
    entity: web::Json<UserProductModel>,
) -> Result<impl Responder, actix_web::Error> {
    let result =
        ProductsService::get_products(connection.as_ref().clone(), &entity.into_inner().username)
            .await;
    Ok(HttpResponse::Ok().json(result))
}

#[post("/user_products_claims")]
pub async fn products_claims(
    connection: web::Data<PgPool>,
    entity: web::Json<UserProductModel>,
) -> Result<impl Responder, actix_web::Error> {
    let user_product = entity.into_inner();

    let result = ProductsService::get_claims(
        connection.as_ref().clone(),
        &user_product.username,
        &user_product.product,
    )
    .await;
    Ok(HttpResponse::Ok().json(result))
}

pub fn auth_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/auth")
            .service(login)
            .service(products)
            .service(products_claims),
    );
}
