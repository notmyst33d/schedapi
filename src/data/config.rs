use std::path::Path;

use serde::Deserialize;

#[derive(Deserialize)]
pub struct ConfigMain {
    pub port: u16,
}

#[derive(Deserialize)]
pub struct ConfigDatabase {
    pub host: String,
    pub port: u16,
    pub ssl_cert: Option<Box<Path>>,
    pub keyspace: String,
    pub user: Option<String>,
    pub password: Option<String>,
}

#[derive(Deserialize)]
pub struct Config {
    pub main: ConfigMain,
    pub database: ConfigDatabase,
}
