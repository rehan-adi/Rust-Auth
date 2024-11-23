use serde::Serialize;

#[derive(Serialize)]
pub struct UserResponse {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub is_login: Option<bool>,
}
