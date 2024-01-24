use serde::Deserialize;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct GenericAccessTokenRequest {
    #[serde(
        default,
        deserialize_with = "crate::serialization::deserialize_uuid_option"
    )]
    pub access_token: Option<Uuid>,
}
