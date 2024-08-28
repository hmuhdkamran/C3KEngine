use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct GatewayConfig {
    pub port: String,
    pub host: String,
    pub cors: Vec<String>
}