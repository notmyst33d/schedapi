use serde::Deserialize;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct GroupDeleteRequest {
    #[serde(deserialize_with = "crate::serialization::deserialize_uuid")]
    pub access_token: Uuid,
    #[serde(deserialize_with = "crate::serialization::deserialize_uuid")]
    pub group_id: Uuid,
}
