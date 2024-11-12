use actix_web::{web, App, HttpServer};
use sqlx::postgres::PgPool;
use dotenv::dotenv;

mod api;

#[tokio::main]
async fn main() -> std::io::Result<()> {

    dotenv().ok();

    let port = std::env::var("PORT").expect("PORT not found in env");
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL not found in env");

    let pool = PgPool::connect(&database_url).await
    .expect("Failed to create database connection pool");

    println!("Server is running on port {}", port);

    HttpServer::new( move || {
           App::new()
           .app_data(web::Data::new(pool.clone()))
           .service(api::healthcheck::health_check)
           .service(api::auth::signup)
    })
    .bind(format!("localhost:{}", port))?
    .run()
    .await
}
