use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct TokenProviderConfig {
    pub token_expiration: u64,
    pub token_security_key: String,
    pub token_security_algorithm: String,
    pub token_issuer: String,
    pub token_audience: String,
    pub salt_length: u32,
}