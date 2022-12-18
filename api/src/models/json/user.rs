use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct RegisterUserSchema {
    pub username: String,
    pub password: String,
    pub email: String,
}

#[derive(Deserialize, Debug)]
pub struct LoginUserSchema {
    pub email: String,
    pub password: String,
}
