use serde::{Serialize, Deserialize};

#[derive(Serialize)]
pub struct LoginSuccess {
    pub code: u16,
    pub access_token: String,
    pub refresh_token: String,
    pub message: &'static str,
    pub success: bool,
}

#[derive(Serialize)]
pub struct RefreshSuccess {
    pub code: u16,
    pub access_token: String,
    pub message: &'static str,
    pub success: bool,
}

#[derive(Deserialize)]
pub struct RefreshTokenSchema {
    pub refresh_token: String,
}
