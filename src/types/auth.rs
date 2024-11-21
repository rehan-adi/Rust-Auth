use validator::Validate;
use serde::{Deserialize, Serialize};

#[derive(Validate, Deserialize)]
pub struct SignupData {
    #[validate(length(min = 2, message = "Username must be at least 2 characters long"))]
    pub username: String,

    #[validate(email(message = "Invalid email format"))]
    pub email: String,

    #[validate(length(min = 6, message = "Password must be at least 6 characters long"))]
    pub password: String,
}

#[derive(Validate, Deserialize)]
pub struct SigninData {
    #[validate(email(message = "Invalid email format"))]
    pub email: String,

    #[validate(length(min = 6, message = "Password must be at least 6 characters long"))]
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: usize, 
}