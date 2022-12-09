use serde::{Deserialize, Serialize};
pub use super::super::database::NewUser;

#[derive(Serialize, Deserialize, Debug)]
pub struct LoginSchema {
    pub email: String,
    pub password: String,
}
