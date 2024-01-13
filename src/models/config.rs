use serde::Deserialize;

#[derive(Deserialize)]
pub struct Config {
    pub port: u16,
    pub schedule: String,
}
