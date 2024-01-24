use serde::{de, Deserialize, Deserializer};
use uuid::Uuid;

pub fn deserialize_uuid<'a, D>(deserializer: D) -> Result<Uuid, D::Error>
where
    D: Deserializer<'a>,
{
    let data: &str = Deserialize::deserialize(deserializer)?;
    Uuid::parse_str(data).map_err(|_| de::Error::custom("Could not parse UUID"))
}

pub fn deserialize_uuid_option<'a, D>(deserializer: D) -> Result<Option<Uuid>, D::Error>
where
    D: Deserializer<'a>,
{
    #[derive(Deserialize)]
    struct Helper(#[serde(deserialize_with = "deserialize_uuid")] Uuid);

    let helper = Option::deserialize(deserializer)?;
    Ok(helper.map(|Helper(external)| external))
}
