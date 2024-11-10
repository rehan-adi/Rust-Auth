use actix_web::{get, HttpResponse, Responder};
use chrono::Utc;
use serde::Serialize;

#[derive(Serialize)]
struct HealthCheckResponse {
    status: String,
    message: String,
    timestamp: String,
}

#[get("/")]
pub async fn health_check() -> impl Responder {
    
    let response = HealthCheckResponse {
        status: "success".to_string(),
        message: "Service is up and running".to_string(),
        timestamp: Utc::now().to_rfc3339(),
    };

    HttpResponse::Ok()
        .json(response)
}
