use serde::Deserialize;

#[derive(Deserialize)]
pub struct LoginInfo {
    pub email: String,
    pub password: String,
}
