use crate::{
    models::payroll::tax_adjustment_approvals::TaxAdjustmentApprovals,
    services::payroll::tax_adjustment_approvals::TaxAdjustmentApprovalsService,
};
use actix_web::{delete, get, post, put, web, HttpResponse, Responder};
use c3k_common::interfaces::iservice::IService;
use sqlx::PgPool;

#[get("")]
pub async fn get_all(connection: web::Data<PgPool>) -> Result<impl Responder, actix_web::Error> {
    let entities = TaxAdjustmentApprovalsService::get_all(connection.as_ref().clone()).await;
    Ok(HttpResponse::Ok().json(entities))
}

#[get("/{id}")]
pub async fn get_by_filter(
    connection: web::Data<PgPool>,
    filter: web::Path<String>,
) -> Result<impl Responder, actix_web::Error> {
    let entities = TaxAdjustmentApprovalsService::get_by_filter(connection.as_ref().clone(), &filter).await;
    Ok(HttpResponse::Ok().json(entities))
}

#[post("")]
pub async fn add(
    connection: web::Data<PgPool>,
    entity: web::Json<TaxAdjustmentApprovals>,
) -> Result<impl Responder, actix_web::Error> {
    let result = TaxAdjustmentApprovalsService::add(connection.as_ref().clone(), &entity.into_inner()).await;
    Ok(HttpResponse::Ok().json(result))
}

#[put("")]
pub async fn update(
    connection: web::Data<PgPool>,
    entity: web::Json<TaxAdjustmentApprovals>,
) -> Result<impl Responder, actix_web::Error> {
    let result = TaxAdjustmentApprovalsService::update(connection.as_ref().clone(), &entity.into_inner()).await;
    Ok(HttpResponse::Ok().json(result))
}

#[delete("/{id}")]
pub async fn delete(
    connection: web::Data<PgPool>,
    id: web::Path<String>,
) -> Result<impl Responder, actix_web::Error> {
    let result = TaxAdjustmentApprovalsService::delete(connection.as_ref().clone(), &id).await;
    Ok(HttpResponse::Ok().json(result))
}

pub fn tax_adjustment_approvals_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/hrms/tax_adjustment_approvals")
            .service(get_all)
            .service(get_by_filter)
            .service(add)
            .service(update)
            .service(delete),
    );
}
