use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct ServiceConfig {
    pub name: String,
    pub connection_string: String,
    pub host: String,
    pub port: String,
}