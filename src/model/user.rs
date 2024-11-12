use sqlx::FromRow;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, FromRow)]
pub struct User {
    pub name: String,
    pub email: String,
    pub password: String,
    pub is_login: bool
}