use crate::{
    models::setup::cities::Cities,
    services::setup::cities::CitiesService,
};
use actix_web::{delete, get, post, put, web, HttpResponse, Responder};
use c3k_common::interfaces::iservice::IService;
use sqlx::PgPool;

#[get("")]
pub async fn get_all(connection: web::Data<PgPool>) -> Result<impl Responder, actix_web::Error> {
    let entities = CitiesService::get_all(connection.as_ref().clone()).await;
    Ok(HttpResponse::Ok().json(entities))
}

#[get("/{id}")]
pub async fn get_by_filter(
    connection: web::Data<PgPool>,
    filter: web::Path<String>,
) -> Result<impl Responder, actix_web::Error> {
    let entities = CitiesService::get_by_filter(connection.as_ref().clone(), &filter).await;
    Ok(HttpResponse::Ok().json(entities))
}

#[post("")]
pub async fn add(
    connection: web::Data<PgPool>,
    entity: web::Json<Cities>,
) -> Result<impl Responder, actix_web::Error> {
    let result = CitiesService::add(connection.as_ref().clone(), &entity.into_inner()).await;
    Ok(HttpResponse::Ok().json(result))
}

#[put("")]
pub async fn update(
    connection: web::Data<PgPool>,
    entity: web::Json<Cities>,
) -> Result<impl Responder, actix_web::Error> {
    let result = CitiesService::update(connection.as_ref().clone(), &entity.into_inner()).await;
    Ok(HttpResponse::Ok().json(result))
}

#[delete("/{id}")]
pub async fn delete(
    connection: web::Data<PgPool>,
    id: web::Path<String>,
) -> Result<impl Responder, actix_web::Error> {
    let result = CitiesService::delete(connection.as_ref().clone(), &id).await;
    Ok(HttpResponse::Ok().json(result))
}

pub fn cities_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/auth/setup/cities")
            .service(get_all)
            .service(get_by_filter)
            .service(add)
            .service(update)
            .service(delete),
    );
}
