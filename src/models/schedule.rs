use scylla::FromUserType;
use scylla::SerializeCql;
use serde::Serialize;

#[derive(Serialize, SerializeCql, FromUserType, Debug)]
pub struct Range {
    pub start: i32,
    pub end: i32,
}

#[derive(Serialize, SerializeCql, FromUserType, Debug)]
pub struct Schedule {
    pub day: i32,
    pub num: i32,
    pub week_range: Range,
    pub name: String,
    pub lesson_type: Option<String>,
    pub teacher: Option<String>,
    pub auditorium: String,
    pub even: Option<bool>,
    pub odd: Option<bool>,
}
