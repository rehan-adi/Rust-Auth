use std::env;
use dotenv::dotenv;
use actix_web::{web, App, HttpServer};

mod routes;
mod config;
mod models;
mod types;
mod schema;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let pool = config::db::db_connect();

    let port = env::var("PORT").expect("Failed to get PORT");
    println!("Server is running on {}", port);

    HttpServer::new(move || {
        App::new()
        .app_data(web::Data::new(pool.clone()))
        .service(routes::healthcheck::health_check)
        .service(routes::auth::signup)
    })
    .bind(format!("localhost:{}", port))?
    .run()
    .await
}
