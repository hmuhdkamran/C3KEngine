use crate::{
    models::attendance::attendamce_exclude_employees::AttendamceExcludeEmployees,
    services::attendance::attendamce_exclude_employees::AttendamceExcludeEmployeesService,
};
use actix_web::{delete, get, post, put, web, HttpResponse, Responder};
use c3k_common::interfaces::iservice::IService;
use sqlx::PgPool;

#[get("")]
pub async fn get_all(connection: web::Data<PgPool>) -> Result<impl Responder, actix_web::Error> {
    let entities = AttendamceExcludeEmployeesService::get_all(connection.as_ref().clone()).await;
    Ok(HttpResponse::Ok().json(entities))
}

#[get("/{id}")]
pub async fn get_by_filter(
    connection: web::Data<PgPool>,
    filter: web::Path<String>,
) -> Result<impl Responder, actix_web::Error> {
    let entities = AttendamceExcludeEmployeesService::get_by_filter(connection.as_ref().clone(), &filter).await;
    Ok(HttpResponse::Ok().json(entities))
}

#[post("")]
pub async fn add(
    connection: web::Data<PgPool>,
    entity: web::Json<AttendamceExcludeEmployees>,
) -> Result<impl Responder, actix_web::Error> {
    let result = AttendamceExcludeEmployeesService::add(connection.as_ref().clone(), &entity.into_inner()).await;
    Ok(HttpResponse::Ok().json(result))
}

#[put("")]
pub async fn update(
    connection: web::Data<PgPool>,
    entity: web::Json<AttendamceExcludeEmployees>,
) -> Result<impl Responder, actix_web::Error> {
    let result = AttendamceExcludeEmployeesService::update(connection.as_ref().clone(), &entity.into_inner()).await;
    Ok(HttpResponse::Ok().json(result))
}

#[delete("/{id}")]
pub async fn delete(
    connection: web::Data<PgPool>,
    id: web::Path<String>,
) -> Result<impl Responder, actix_web::Error> {
    let result = AttendamceExcludeEmployeesService::delete(connection.as_ref().clone(), &id).await;
    Ok(HttpResponse::Ok().json(result))
}

pub fn attendamce_exclude_employees_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/hrms/attendamce_exclude_employees")
            .service(get_all)
            .service(get_by_filter)
            .service(add)
            .service(update)
            .service(delete),
    );
}
