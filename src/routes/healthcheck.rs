use serde:: Serialize;
use actix_web::{ get, HttpResponse, Responder };

#[derive(Serialize)]
struct ResponseType {
    status: &'static str, 
    message: &'static str
}

#[get("/")]
pub async fn health_check() -> impl Responder  {
    return HttpResponse::Ok().json(ResponseType {
        status: "OK",
        message: "Server is healthy"
    });
}