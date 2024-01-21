use serde::Serialize;

#[derive(Serialize)]
pub struct UserTokenResponse {
    pub access_token: String,
}
