use serde::{de, Deserialize, Deserializer, Serialize, Serializer};
use sqlx::types::Json;
use sqlx::FromRow;
use tokio::sync::RwLock;
use utoipa::ToSchema;

use crate::storage::Storage;

#[derive(Deserialize, Debug)]
pub struct PortableScheduleEntry {
    pub day: u8,
    pub num: u8,
    #[serde(deserialize_with = "from_week_ranges")]
    pub week_ranges: Vec<Range>,
    pub name: String,
    pub lesson_type: Option<String>,
    pub teacher: Option<String>,
    pub auditorium: String,
    pub even_odd: EvenOdd,
}

impl Into<Schedule> for PortableScheduleEntry {
    fn into(self) -> Schedule {
        Schedule {
            day: self.day as i32,
            num: self.num as i32,
            week_ranges: self.week_ranges,
            name: self.name,
            lesson_type: self.lesson_type,
            teacher: self.teacher,
            auditorium: self.auditorium,
            even_odd: self.even_odd,
        }
    }
}

fn from_week_ranges<'a, D>(deserializer: D) -> Result<Vec<Range>, D::Error>
where
    D: Deserializer<'a>,
{
    let data: &str = Deserialize::deserialize(deserializer)?;
    let weeks: Vec<&str> = data.split(",").collect();
    let mut ranges: Vec<Range> = vec![];

    for week in weeks {
        if week == "all" {
            return Ok(vec![Range {
                start: 0,
                end: i32::MAX,
            }]);
        }

        let split: Vec<&str> = week.split("-").collect();
        let start: i32 = split
            .get(0)
            .ok_or(de::Error::custom("No week range start"))
            .map(|v| v.parse::<i32>())?
            .map_err(de::Error::custom)?;

        let mut end = start;
        if !split.get(1).is_none() {
            end = split
                .get(1)
                .ok_or(de::Error::custom("No week range end"))
                .map(|v| v.parse::<i32>())?
                .map_err(de::Error::custom)?;
        }

        ranges.push(Range { start, end })
    }

    Ok(ranges)
}

#[derive(Debug, PartialEq)]
pub enum EvenOdd {
    No = 0,
    Even = 1,
    Odd = 2,
}

impl Serialize for EvenOdd {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            EvenOdd::No => serializer.serialize_str("none"),
            EvenOdd::Even => serializer.serialize_str("true"),
            EvenOdd::Odd => serializer.serialize_str("false"),
        }
    }
}

impl<'de> Deserialize<'de> for EvenOdd {
    fn deserialize<D>(deserializer: D) -> Result<EvenOdd, D::Error>
    where
        D: Deserializer<'de>,
    {
        Ok(match String::deserialize(deserializer)?.as_str() {
            "Ч" => EvenOdd::Even,
            "Н" => EvenOdd::Odd,
            "true" => EvenOdd::Even,
            "false" => EvenOdd::Odd,
            _ => EvenOdd::No,
        })
    }
}

#[derive(FromRow, Serialize, Deserialize, Debug)]
pub struct Range {
    pub start: i32,
    pub end: i32,
}

#[derive(FromRow, Serialize, Deserialize, Debug)]
pub struct Schedule {
    pub day: i32,
    pub num: i32,
    pub week_ranges: Vec<Range>,
    pub name: String,
    pub lesson_type: Option<String>,
    pub teacher: Option<String>,
    pub auditorium: String,
    pub even_odd: EvenOdd,
}

#[derive(FromRow, Serialize, Deserialize, Debug)]
pub struct Group {
    pub id: i64,
    pub epoch: Option<i64>,
    pub name: String,
    pub schedule: Option<Json<Vec<Schedule>>>,
}

#[derive(Serialize)]
pub enum ApiResponse<T> {
    #[serde(rename = "ok")]
    Ok(T),
    #[serde(rename = "error")]
    Err(String),
}

#[derive(Serialize, ToSchema)]
pub struct ScheduleEntry {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub lesson_type: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub teacher: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub auditorium: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub empty: Option<bool>,
}

#[derive(Deserialize)]
pub struct Config {
    pub port: u16,
    pub jwt_secret: String,
    pub product: ConfigProduct,
    pub support: Vec<SupportContact>,
}

#[derive(Deserialize)]
pub struct ConfigProduct {
    pub name: String,
    pub logo: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct SupportContact {
    pub name: String,
    pub url: String,
}

pub struct SharedState {
    pub storage: Storage,
    pub jwt_secret: String,
    pub authorized_routes: RwLock<Vec<&'static str>>,
    pub password: RwLock<String>,
    pub product_name: &'static str,
    pub product_logo: &'static [u8],
    pub support: Vec<SupportContact>,
}
