use actix_web::{post, web, HttpResponse, Responder};
use c3k_common::models::auth::{AuthModel, SignupUsers, UserProductModel};
use sqlx::PgPool;

use crate::services::role::{auth_service::AuthService, products::ProductsService};

#[post("/login")]
pub async fn login(
    service: web::Data<AuthService>,
    entity: web::Json<AuthModel>,
) -> Result<impl Responder, actix_web::Error> {
    let result = service.login(&entity.into_inner()).await;

    Ok(HttpResponse::Ok().json(result))
}

#[post("/user_signup")]
pub async fn signup(
    service: web::Data<AuthService>,
    entity: web::Json<SignupUsers>,
) -> Result<impl Responder, actix_web::Error> {
    let result = service.signup(&entity.into_inner()).await;

    Ok(HttpResponse::Ok().json(result))
}

#[post("/logout")]
pub async fn logout(
    service: web::Data<AuthService>,
    entity: web::Json<AuthModel>,
) -> Result<impl Responder, actix_web::Error> {
    let result = service.logout(&entity.into_inner()).await;

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

#[post("/user_product_claims")]
pub async fn product_claims(
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
            .service(signup)
            .service(logout)
            .service(products)
            .service(product_claims),
    );
}
