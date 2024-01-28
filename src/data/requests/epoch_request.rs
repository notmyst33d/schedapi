use serde::Deserialize;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct EpochRequest {
    #[serde(
        default,
        deserialize_with = "crate::serialization::deserialize_uuid_option"
    )]
    pub group_id: Option<Uuid>,
}
