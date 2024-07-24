use serde::Deserialize;
use serde_json::from_str;
use std::error::Error;
use std::fs;

use super::{
    backend_config::BackendEngineConfig, data_config::DataConfig, ftp_config::FtpConfig,
    radis_config::RedisConfig, toekn_provider_config::TokenProviderConfig,
};

#[derive(Debug, Deserialize, Clone)]
pub struct AppConfig {
    pub data: DataConfig,
    pub redis: RedisConfig,
    pub ftp: FtpConfig,
    pub claims_namespace: String,
    pub token_provider: TokenProviderConfig,
    pub backend_engine: BackendEngineConfig,
}

pub fn get_json() -> Result<AppConfig, Box<dyn Error>> {
    let config_contents = fs::read_to_string("app.development.json")?;
    let app_config: AppConfig = from_str(&config_contents)?;

    Ok(app_config.clone())
}
