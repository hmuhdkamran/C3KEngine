use crate::{
    models::point_of_sale::sale_order_customer_points::SaleOrderCustomerPoints,
    services::point_of_sale::sale_order_customer_points::SaleOrderCustomerPointsService,
};
use actix_web::{delete, get, post, put, web, HttpResponse, Responder};
use c3k_common::interfaces::iservice::IService;
use sqlx::PgPool;

#[get("")]
pub async fn get_all(connection: web::Data<PgPool>) -> Result<impl Responder, actix_web::Error> {
    let entities = SaleOrderCustomerPointsService::get_all(connection.as_ref().clone()).await;
    Ok(HttpResponse::Ok().json(entities))
}

#[get("/{id}")]
pub async fn get_by_filter(
    connection: web::Data<PgPool>,
    filter: web::Path<String>,
) -> Result<impl Responder, actix_web::Error> {
    let entities = SaleOrderCustomerPointsService::get_by_filter(connection.as_ref().clone(), &filter).await;
    Ok(HttpResponse::Ok().json(entities))
}

#[post("")]
pub async fn add(
    connection: web::Data<PgPool>,
    entity: web::Json<SaleOrderCustomerPoints>,
) -> Result<impl Responder, actix_web::Error> {
    let result = SaleOrderCustomerPointsService::add(connection.as_ref().clone(), &entity.into_inner()).await;
    Ok(HttpResponse::Ok().json(result))
}

#[put("")]
pub async fn update(
    connection: web::Data<PgPool>,
    entity: web::Json<SaleOrderCustomerPoints>,
) -> Result<impl Responder, actix_web::Error> {
    let result = SaleOrderCustomerPointsService::update(connection.as_ref().clone(), &entity.into_inner()).await;
    Ok(HttpResponse::Ok().json(result))
}

#[delete("/{id}")]
pub async fn delete(
    connection: web::Data<PgPool>,
    id: web::Path<String>,
) -> Result<impl Responder, actix_web::Error> {
    let result = SaleOrderCustomerPointsService::delete(connection.as_ref().clone(), &id).await;
    Ok(HttpResponse::Ok().json(result))
}

pub fn sale_order_customer_points_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/retail/sale_order_customer_points")
            .service(get_all)
            .service(get_by_filter)
            .service(add)
            .service(update)
            .service(delete),
    );
}
