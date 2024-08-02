use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct BackendEngineConfig {
    pub port: String,
    pub host: String,
    pub cors: Vec<String>
}