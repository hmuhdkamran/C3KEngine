use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct RedisConfig {
    pub redis_host: String,
    pub redis_port: String,
}