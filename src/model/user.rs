use serde::{Deserialize, Serialize};

#[derive(Deserialize,Serialize, Debug)]
pub struct User {
    pub name: String,
    pub email: String,
    pub password: String,
    pub is_login: bool
}