use once_cell::sync::Lazy;
use serde::Deserialize;
use sqlx::{postgres::PgPoolOptions, PgPool, Row};
use std::fs;
use std::sync::Mutex;

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

// A globally accessible singleton for AppConfig
static APP_CONFIG: Lazy<Mutex<Option<AppConfig>>> = Lazy::new(|| Mutex::new(None));

/// Initialize and load the AppConfig once.
pub async fn initialize_config() -> Result<(), Box<dyn std::error::Error>> {
    let mut config = APP_CONFIG.lock().unwrap();

    if config.is_none() {
        let new_config = load_app_config().await?;
        *config = Some(new_config);
    }

    Ok(())
}

/// Access the AppConfig singleton. Returns a clone of the config.
pub fn get_config() -> Option<AppConfig> {
    APP_CONFIG.lock().unwrap().clone()
}

/// Load the configuration from JSON and the database
async fn load_app_config() -> Result<AppConfig, Box<dyn std::error::Error>> {
    // Read the configuration file
    let config_contents = fs::read_to_string("app.development.json")
        .map_err(|e| format!("Failed to read configuration file: {}", e))?;

    // Parse JSON into AppConfig
    let mut app_config: AppConfig = serde_json::from_str(&config_contents)
        .map_err(|e| format!("Failed to parse JSON: {}", e))?;

    // Find the service with name "api/auth"
    let auth_service = app_config
        .services
        .iter()
        .find(|service| service.name == "api/auth")
        .ok_or("Service with name 'api/auth' not found")?;

    // Create a database pool using the async sqlx client
    let db_pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&auth_service.connection_string)
        .await
        .map_err(|e| format!("Failed to create database pool: {}", e))?;

    // Execute the query
    let rows = sqlx::query(
        r#"SELECT "ApiPrefix", "ConnectionString", "HostIp", "HostPort"
           FROM "Role"."Products"
           WHERE "ApiPrefix" NOT ILIKE 'api/auth'"#,
    )
    .fetch_all(&db_pool)
    .await
    .map_err(|e| format!("Failed to execute query: {}", e))?;

    // Process query results
    for row in rows {
        let service_config = ServiceConfig {
            name: row.try_get("ApiPrefix")?,
            connection_string: row.try_get("ConnectionString")?,
            host: row.try_get("HostIp")?,
            port: row.try_get::<i32, _>("HostPort")?.to_string(),
        };

        app_config.services.push(service_config);
    }

    Ok(app_config)
}

pub async fn create_db_pool(connection_string: &str) -> Result<PgPool, sqlx::Error> {
    match PgPoolOptions::new()
        .max_connections(5)
        .connect(connection_string)
        .await
    {
        Ok(pool) => Ok(pool),
        Err(e) => {
            eprintln!("Failed to create database pool: {}", e);
            Err(e)
        }
    }
}
