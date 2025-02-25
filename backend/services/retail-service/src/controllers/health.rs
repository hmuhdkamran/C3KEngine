use actix_web::{get, web, HttpResponse, Responder};
use sqlx::{Executor, PgPool};

#[get("/health")]
pub async fn health_check(pool: web::Data<PgPool>) -> impl Responder {
    match pool.fetch_one("SELECT 1").await {
        Ok(_) => HttpResponse::Ok().body("Healthy"),
        Err(_) => HttpResponse::InternalServerError().body("Database connection failed"),
    }
}

pub fn health_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/retail")
            .service(health_check),
    );
}
