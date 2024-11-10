use actix_web::{App, HttpServer};
use dotenv::dotenv;

mod api;

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    dotenv().ok();

    let port = std::env::var("PORT").expect("ENV not found");
    println!("Server is running on port {}", port);

    HttpServer::new(|| {
           App::new().service(api::healthcheck::health_check)
    })
    .bind(format!("localhost:{}", port))?
    .run()
    .await
}
