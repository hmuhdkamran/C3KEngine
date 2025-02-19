use crate::{
    models::setup::ownership_status::OwnershipStatus,
    services::setup::ownership_status::OwnershipStatusService,
};
use actix_web::{delete, get, post, put, web, HttpResponse, Responder};
use c3k_common::interfaces::iservice::IService;
use sqlx::PgPool;

#[get("")]
pub async fn get_all(connection: web::Data<PgPool>) -> Result<impl Responder, actix_web::Error> {
    let entities = OwnershipStatusService::get_all(connection.as_ref().clone()).await;
    Ok(HttpResponse::Ok().json(entities))
}

#[get("/{id}")]
pub async fn get_by_filter(
    connection: web::Data<PgPool>,
    filter: web::Path<String>,
) -> Result<impl Responder, actix_web::Error> {
    let entities = OwnershipStatusService::get_by_filter(connection.as_ref().clone(), &filter).await;
    Ok(HttpResponse::Ok().json(entities))
}

#[post("")]
pub async fn add(
    connection: web::Data<PgPool>,
    entity: web::Json<OwnershipStatus>,
) -> Result<impl Responder, actix_web::Error> {
    let result = OwnershipStatusService::add(connection.as_ref().clone(), &entity.into_inner()).await;
    Ok(HttpResponse::Ok().json(result))
}

#[put("")]
pub async fn update(
    connection: web::Data<PgPool>,
    entity: web::Json<OwnershipStatus>,
) -> Result<impl Responder, actix_web::Error> {
    let result = OwnershipStatusService::update(connection.as_ref().clone(), &entity.into_inner()).await;
    Ok(HttpResponse::Ok().json(result))
}

#[delete("/{id}")]
pub async fn delete(
    connection: web::Data<PgPool>,
    id: web::Path<String>,
) -> Result<impl Responder, actix_web::Error> {
    let result = OwnershipStatusService::delete(connection.as_ref().clone(), &id).await;
    Ok(HttpResponse::Ok().json(result))
}

pub fn ownership_status_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/auth/setup/ownership_status")
            .service(get_all)
            .service(get_by_filter)
            .service(add)
            .service(update)
            .service(delete),
    );
}
