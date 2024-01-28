use serde::Serialize;
use scylla::SerializeCql;
use scylla::FromUserType;

#[derive(Serialize, SerializeCql, FromUserType, Debug)]
pub struct Range {
    pub start: i32,
    pub end: i32,
}
