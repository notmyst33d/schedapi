use serde::{de, Deserialize, Deserializer};
use uuid::Uuid;

pub fn deserialize_uuid<'a, D>(deserializer: D) -> Result<Uuid, D::Error>
where
    D: Deserializer<'a>,
{
    let data: &str = Deserialize::deserialize(deserializer)?;
    Uuid::parse_str(data).map_err(|_| de::Error::custom("Could not parse UUID"))
}
