use serde::Deserialize;

#[derive(Deserialize)]
pub struct UserTokenRequest {
    pub username: String,
    pub password: String,
}
