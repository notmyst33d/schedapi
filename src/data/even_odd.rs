use serde::{Serialize, Deserialize};
use scylla::{SerializeCql, FromUserType};

pub enum Value {
    NONE = 0,
    EVEN = 1,
    ODD = 2,
}

#[derive(Serialize, Deserialize, FromUserType, SerializeCql, Debug)]
pub struct EvenOdd {
    pub value: i32,
}

