use scylla::{FromUserType, SerializeCql};
use serde::{Deserialize, Serialize};

pub enum Value {
    NONE = 0,
    EVEN = 1,
    ODD = 2,
}

#[derive(Serialize, Deserialize, FromUserType, SerializeCql, Debug)]
pub struct EvenOdd {
    pub value: i32,
}
