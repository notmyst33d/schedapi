use serde::Deserialize;

#[derive(Deserialize)]
pub struct ImportRequest {
    pub group: String,
    pub file: Vec<u8>,
}
