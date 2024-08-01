use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct DataConfig {
    pub connection_string: Vec<String>,
}