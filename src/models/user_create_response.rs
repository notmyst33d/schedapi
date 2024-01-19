use serde::Serialize;

#[derive(Serialize)]
pub struct UserCreateResponse {
    pub access_token: String,
}
