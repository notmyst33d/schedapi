use serde::Deserialize;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct ScheduleRequest {
    pub week: i32,
    #[serde(deserialize_with = "crate::serialization::deserialize_uuid")]
    pub group_id: Uuid,
}
