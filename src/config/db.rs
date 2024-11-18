use std::env;
use diesel::PgConnection;
use diesel::r2d2::{self, ConnectionManager}; 

pub type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub fn db_connect() -> DbPool {

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let pool = ConnectionManager::<PgConnection>::new(database_url);

    r2d2::Pool::builder()
    .build(pool)
    .expect("Failed to create connection pool")

}