use serde::Deserialize;

#[derive(Deserialize)]
pub struct RegisterInfo {
    pub username: String,
    pub email: String,
    pub password: String,
}
