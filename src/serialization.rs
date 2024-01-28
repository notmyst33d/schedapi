use serde::{de, Deserialize, Deserializer};
use uuid::Uuid;

use crate::data::{EvenOdd, EvenOddValue};

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

pub fn deserialize_even_odd<'a, D>(deserializer: D) -> Result<EvenOdd, D::Error>
where
    D: Deserializer<'a>,
{
    let data: &str = Deserialize::deserialize(deserializer)?;
    let value = match data {
        "Ч" => EvenOddValue::EVEN as i32,
        "Н" => EvenOddValue::ODD as i32,
        "true" => EvenOddValue::EVEN as i32,
        "false" => EvenOddValue::ODD as i32,
        _ => EvenOddValue::NONE as i32,
    };

    Ok(EvenOdd { value })
}
