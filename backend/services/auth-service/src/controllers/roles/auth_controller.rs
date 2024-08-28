use actix_web::{post, web, HttpResponse, Responder};
use c3k_common::{handler::redis_handler::RedisHandler, models::auth::AuthModel};
use sqlx::PgPool;

use crate::services::auth_service::AuthService;

#[post("")]
pub async fn login(
    connection: web::Data<PgPool>,
    entity: web::Json<AuthModel>,
) -> Result<impl Responder, actix_web::Error> {
    let redis_client = RedisHandler::new()?;
    let service = AuthService::new(connection.as_ref().clone(), redis_client);
    let result = service.login(&entity.into_inner()).await;
    Ok(HttpResponse::Ok().json(result))
}

pub fn auth_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/api/auth").service(login));
}
