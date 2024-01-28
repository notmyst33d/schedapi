use serde::Deserialize;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct EpochUpdateRequest {
    #[serde(deserialize_with = "crate::serialization::deserialize_uuid")]
    pub access_token: Uuid,
    pub epoch: i64,
    #[serde(
        default,
        deserialize_with = "crate::serialization::deserialize_uuid_option"
    )]
    pub group_id: Option<Uuid>,
}
