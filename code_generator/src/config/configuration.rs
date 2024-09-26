use serde::Deserialize;
use serde_json::from_str;

use std::{ fs, error::Error };

#[derive(Debug, Deserialize, Clone)]
pub struct DataType {
    pub postgresql: String,
    pub rust: String,
    pub typescript: String
}

#[derive(Debug, Deserialize, Clone)]
pub struct Modules {
    pub property_name: String,
    pub file_path: String,
    pub property_string: String,
    pub distination: String,
    pub language: String,
    pub extension: String,
    pub export_text: String,
    pub common_file: String
}

#[derive(Debug, Deserialize, Clone)]
pub struct ColumnConfiguration {
    pub property_name: String,
    pub property_string: String,
    pub display_type: String,
    pub recursive: String,
    pub horizontal: String,
    pub ends_with: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Configuration {
    pub connection_string: String,
    pub service: String,
    pub base_path: String,
    pub module_configuration: Vec<Modules>,
    pub column_configurations: Vec<ColumnConfiguration>,
    pub data_types: Vec<DataType>
}

pub fn get_json() -> Result<Configuration, Box<dyn Error>> {
    let config_contents = fs::read_to_string("store/app.configuration.json")?;
    let app_config: Configuration = from_str(&config_contents)?;

    Ok(app_config.clone())
}