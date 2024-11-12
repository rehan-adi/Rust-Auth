use sqlx::PgPool;
use serde::Deserialize;
use bcrypt::{hash, DEFAULT_COST};
use actix_web::{post, web, HttpResponse, Responder};

#[derive(Deserialize)]
struct SignUpData {
    name: String,
    email: String,
    password: String
}

#[post("/signup")]
pub async fn signup(
   pool: web::Data<PgPool>, 
   data: web::Json<SignUpData>
) -> impl Responder {

   let hashed_password = match hash(&data.password, DEFAULT_COST) {
      Ok(hashed) => hashed,
      Err(_) => return HttpResponse::InternalServerError().body("Error hashing password"),
  };

     let response = sqlx::query!(
        "INSERT INTO users (name, email, password, is_login) VALUES ($1, $2, $3, $4)",
        data.name,
        data.email,
        hashed_password,
        false
     )
     .execute(pool.get_ref())
     .await;

     match response {
        Ok(_) => HttpResponse::Ok().body("User created successfully"),
        Err(_) => HttpResponse::InternalServerError().body("Failed to create user"),
     }

}
