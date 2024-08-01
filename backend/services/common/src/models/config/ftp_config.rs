use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct FtpConfig {
    pub server: String,
    pub username: String,
    pub password: String,
}