use scylla::FromUserType;
use scylla::SerializeCql;
use serde::Serialize;

use crate::data::EvenOdd;
use crate::data::Range;

#[derive(Serialize, SerializeCql, FromUserType, Debug)]
pub struct Schedule {
    pub day: i32,
    pub num: i32,
    pub week_ranges: Vec<Range>,
    pub name: String,
    pub lesson_type: Option<String>,
    pub teacher: Option<String>,
    pub auditorium: String,
    #[serde(deserialize_with = "crate::serialization::deserialize_even_odd")]
    pub even_odd: EvenOdd,
}
