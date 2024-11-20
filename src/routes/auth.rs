use diesel::RunQueryDsl;
use bcrypt::{hash, DEFAULT_COST};
use crate::types::auth::SignupData;
use actix_web::{post, web, HttpResponse, Responder};
use crate::{config::db::DbPool, models::user::NewUser, schema::users};

#[post("/signup")]
pub async fn signup(
    pool: web::Data<DbPool>, 
    data: web::Json<SignupData>
) -> impl Responder {

    let conn = &mut pool.get().expect("Failed to get DB connection");

    let hash_password = hash(&data.password, DEFAULT_COST).unwrap();

    let new_user = NewUser {
        username: &data.username,
        email: &data.email,
        password: &hash_password,
        is_login: Some(false),
    };

    match diesel::insert_into(users::table)
    .values(&new_user)
    .execute(conn)
{
    Ok(_) => HttpResponse::Created().body("User created successfully"),
    Err(_) => HttpResponse::InternalServerError().body("Error creating user"),
}

}