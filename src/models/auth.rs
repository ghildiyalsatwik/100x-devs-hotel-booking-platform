use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct SignupRequest {
    pub name: Option<String>,
    pub email: Option<String>,
    pub password: Option<String>,
    pub role: Option<String>,
    pub phone: Option<String>,
}

#[derive(Serialize)]
pub struct SignupResponse {
    pub id: String,
    pub name: String,
    pub email: String,
    pub role: String,
    pub phone: Option<String>,
}