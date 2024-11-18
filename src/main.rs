use std::env;
use dotenv::dotenv;
use actix_web::{web, App, HttpServer};

mod routes;
mod config;

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
    })
    .bind(format!("localhost:{}", port))?
    .run()
    .await
}
