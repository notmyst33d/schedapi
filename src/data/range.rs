use scylla::FromUserType;
use scylla::SerializeCql;
use serde::Serialize;

#[derive(Serialize, SerializeCql, FromUserType, Debug)]
pub struct Range {
    pub start: i32,
    pub end: i32,
}
