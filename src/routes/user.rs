use diesel::prelude::*;
use actix_web::{get, web, HttpResponse, Responder};
use crate::{config::db::DbPool, models::user::User, schema::users};

#[get("/details/{email}")]
pub async fn get_details(
    pool: web::Data<DbPool>,
    email: web::Path<String>
) -> impl Responder {

     let conn = &mut pool.get().expect("Failed to get DB connection");

     let email_str = email.as_str();

     let user_result = users::table
     .filter(users::email.eq(&email_str))
     .first::<User>(conn);

     match user_result {
        Ok(user) => {
            HttpResponse::Ok().json(user)
        }
        Err(diesel::result::Error::NotFound) => {
            HttpResponse::NotFound().body("User not found")
        }
        Err(err) => {
            eprintln!("Database error: {:?}", err);
            HttpResponse::InternalServerError().body("Database error")
        }
     }

}