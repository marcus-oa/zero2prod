//! src/routes/health_check.rs

use actix_web::HttpResponse;

pub async fn health_checker() -> HttpResponse {
    HttpResponse::Ok().finish()
}

