use scylla::FromRow;
use scylla::SerializeRow;
use serde::Serialize;
use serde::Serializer;
use uuid::Uuid;

#[derive(Serialize, SerializeRow, FromRow)]
pub struct GroupWithoutSchedule {
    #[serde(serialize_with = "serialize_uuid")]
    pub id: Uuid,
    pub epoch: i64,
    pub name: String,
}

pub fn serialize_uuid<S>(uuid: &Uuid, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_str(uuid.hyphenated().encode_lower(&mut Uuid::encode_buffer()))
}
