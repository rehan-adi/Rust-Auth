use chrono::Utc;
use serde_json::json;
use log::{error, info};
use diesel::prelude::*;
use validator::Validate;
use diesel::associations::HasTable;
use bcrypt::{hash, verify, DEFAULT_COST};
use jsonwebtoken::{encode, Header, EncodingKey};
use crate::schema::users::dsl::{users, email};
use actix_web::{post, web, HttpResponse, Responder};
use crate::types::auth::{SigninData, SignupData, Claims};
use crate::{config::db::DbPool, models::user::{User, NewUser}};

// signup controller 
#[post("/signup")]
pub async fn signup(
    pool: web::Data<DbPool>, 
    data: web::Json<SignupData>
) -> impl Responder {

    //  Validate the input data
    match data.validate() {
        Ok(_) => {
        }
        Err(validation_errors) => {
            error!("Validation failed for signup: {:?}", validation_errors);
            return HttpResponse::BadRequest().json(json!({
                "error": "Invalid input",
                "details": validation_errors
            }));
        }
    }

    // Establish a database connection from the connection pool
    let conn = &mut pool.get().expect("Failed to get DB connection");

    // Check if the user already exists based on email
    let existing_user = users
        .filter(email.eq(&data.email))
        .first::<User>(conn);

    match existing_user {
        Ok(_) => {
            // If the user exists, return a BadRequest with a message
            info!("User already exists with email: {}", &data.email);
            return HttpResponse::BadRequest().json(json!({
                "error": "User already exists",
                "message": "A user with this email already exists."
            }));
        }
        Err(_) => {
            // If no user exists, continue to the next steps
            info!("No existing user with email: {}", &data.email);
        }
    }

    //  Hash the user's password before saving it to the database
    let hash_password = hash(&data.password, DEFAULT_COST).unwrap();

    // Create a new user struct for insertion
    let new_user = NewUser {
        username: &data.username,
        email: &data.email,
        password: &hash_password,
        is_login: Some(false),
    };

    // Insert the new user into the users table
    match diesel::insert_into(users::table())
    .values(&new_user)
    .execute(conn)
{
    Ok(_) => {
        info!("User created successfully with email: {}", &data.email);
        HttpResponse::Created().body("User created successfully")
    }
    Err(err) => {
        error!("Error inserting user into database: {:?}", err);
        HttpResponse::InternalServerError().json(json!( {
            "error": "Error creating user",
            "message": "An error occurred while creating the user."
        }))
    }
  }
}


// signin controller 
#[post("/signin")]
pub async fn signin(
    pool: web::Data<DbPool>,
    data: web::Json<SigninData>
) -> impl Responder {

    match data.validate() {
        Ok(_) => {
        }
        Err(error) => {
            error!("Validation failed for signup: {:?}", error);
            return HttpResponse::BadRequest().json(json!({
                "error": "Invalid input",
                "details": error
            }));
        }
    }

    let conn = &mut pool.get().expect("Failed to get DB connection");

    let user = match users.filter(email.eq(&data.email)).first::<User>(conn) {
        Ok(user) => user,
        Err(diesel::result::Error::NotFound) => {
            error!("No user found with email: {}", data.email);
            return HttpResponse::Unauthorized().body("Invalid email or password");
        }
        Err(err) => {
            error!("Database error while querying user: {:?}", err);
            return HttpResponse::InternalServerError().body("Database error");
        }
    };

    match verify(&data.password, &user.password) {
        Ok(true) => (),
        Ok(false) => return HttpResponse::Unauthorized().body("Invalid email or password"),
        Err(err) => {
            error!("Password verification error: {:?}", err);
            return HttpResponse::InternalServerError().body("Password verification failed");
        }
    }
    
    
    let expiration = Utc::now() + chrono::Duration::seconds(3600); // 1 hour expiration
    let claims = Claims {
        sub: user.email.clone(),
        exp: expiration.timestamp() as usize,
    };

    // Get the JWT secret from environment
    let secret = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");

    let encoding_key = EncodingKey::from_secret(secret.as_ref());
    let token = match encode(&Header::default(), &claims, &encoding_key) {
        Ok(t) => t,
        Err(err) => {
            error!("Failed to create JWT token: {:?}", err);
            return HttpResponse::InternalServerError().body("Failed to create token");
        }
    };

    info!("User signed in successfully: {}", user.email);
    HttpResponse::Ok().json(json!({
        "message": "Signin successful",
        "token": token,
        "user": {
            "id": user.id,
            "username": user.username,
            "email": user.email,
            "is_login": user.is_login.unwrap_or(false),
        }
    }))
    
}