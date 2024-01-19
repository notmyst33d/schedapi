use serde::Deserialize;

#[derive(Deserialize)]
pub struct GroupCreateRequest {
    pub access_token: String,
    pub name: String,
}
