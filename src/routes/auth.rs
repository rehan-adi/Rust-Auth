use chrono::{Utc};
use diesel::prelude::*;
use diesel::associations::HasTable;
use bcrypt::{hash, verify, DEFAULT_COST};
use jsonwebtoken::{encode, Header, EncodingKey};
use crate::schema::users::dsl::{users, email};
use actix_web::{post, web, HttpResponse, Responder};
use crate::types::auth::{SigninData, SignupData, Claims};
use crate::{config::db::DbPool, models::user::{User, NewUser}};

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

    match diesel::insert_into(users::table())
    .values(&new_user)
    .execute(conn)
{
    Ok(_) => HttpResponse::Created().body("User created successfully"),
    Err(_) => HttpResponse::InternalServerError().body("Error creating user"),
}

}


#[post("/signin")]
pub async fn signin(
    pool: web::Data<DbPool>,
    data: web::Json<SigninData>
) -> impl Responder {

    let conn = &mut pool.get().expect("Failed to get DB connection");
    
    let result = users
    .filter(email.eq(&data.email))
    .first::<User>(conn);

    match result {
        Ok(user) => {
            if verify(&data.password, &user.password).unwrap_or(false) {
                
                let expiration = 3600;
                // let claims = Claims {
                //     sub: user.email.clone(),
                //     exp: (Utc::now().timestamp() + expiration) as usize,
                // };

                HttpResponse::Ok().body("Signin successful")
            } else {
                HttpResponse::Unauthorized().body("Invalid email or password")
            }
        }
        Err(_) => {
            HttpResponse::NotFound().body("User not found")
        }
    }

}