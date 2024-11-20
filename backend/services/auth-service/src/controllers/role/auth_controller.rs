use actix_web::{post, web, HttpResponse, Responder};
use c3k_common::{handler::redis_handler::RedisHandler, models::auth::AuthModel};
use sqlx::PgPool;

use crate::services::role::auth_service::AuthService;

#[post("")]
pub async fn login(
    connection: web::Data<PgPool>,
    redis_client: web::Data<RedisHandler>,
    entity: web::Json<AuthModel>,
) -> Result<impl Responder, actix_web::Error> {
    let service = AuthService::new(connection.as_ref().clone(), redis_client.as_ref().clone());
    let result = service.login(&entity.into_inner()).await;

    Ok(HttpResponse::Ok().json(result))
}

pub fn auth_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/api/auth/login").service(login));
}
