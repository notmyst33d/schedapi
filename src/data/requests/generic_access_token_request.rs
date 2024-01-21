use serde::Deserialize;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct GenericAccessTokenRequest {
    #[serde(deserialize_with = "crate::serialization::deserialize_uuid")]
    pub access_token: Uuid,
}
