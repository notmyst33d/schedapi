use serde::Serialize;

#[derive(Serialize)]
pub struct EpochResponse {
    pub epoch: i64,
}
