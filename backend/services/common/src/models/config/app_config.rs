use serde::Deserialize;
use serde_json::from_str;
use std::error::Error;
use std::fs;

use super::{
    ftp_config::FtpConfig, gateway_config::GatewayConfig, radis_config::RedisConfig,
    services_config::ServiceConfig, toekn_provider_config::TokenProviderConfig,
};

#[derive(Debug, Deserialize, Clone)]
pub struct AppConfig {    
    pub redis: RedisConfig,
    pub ftp: FtpConfig,
    pub claims_namespace: String,
    pub token_provider: TokenProviderConfig,
    pub gateway: GatewayConfig,
    pub services: Vec<ServiceConfig>,
}

pub fn get_json() -> Result<AppConfig, Box<dyn Error>> {
    let config_contents = fs::read_to_string("app.development.json")?;
    let app_config: AppConfig = from_str(&config_contents)?;

    Ok(app_config.clone())
}
