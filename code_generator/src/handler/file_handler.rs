use convert_case::{Case, Casing};
use std::fs::{create_dir_all, read_to_string, File, OpenOptions};
use std::io::{self, Write};
use std::path::Path;

use super::postgresql_handler::{Schema, Table};
use crate::config::configuration::{ColumnConfiguration, Configuration};

pub struct FileHandler {
    schemas: Vec<Schema>,
    config: Configuration,
}

impl FileHandler {
    pub fn new(schemas: Vec<Schema>, config: Configuration) -> Self {
        Self { schemas, config }
    }

    pub fn process_files(&self) -> io::Result<()> {
        for module in &self.config.module_configuration {
            // Read the file template
            let distination_path = Path::new(&module.distination);
            if !distination_path.exists() {
                create_dir_all(distination_path)?;
            }

            let file_contents = read_to_string(&module.file_path)?;

            // Create mod.rs at the root of module.distination
            let root_mod_file_path = Path::new(&module.distination).join(&module.common_file);
            let mut root_mod_file = OpenOptions::new()
                .write(true)
                .create(true)
                .append(true)
                .open(&root_mod_file_path)?;

            // Process each schema
            for schema in &self.schemas {
                let schema_folder = format!(
                    "{}/{}",
                    module.distination,
                    schema.schema_name.to_lowercase()
                );
                let schema_path = Path::new(&schema_folder);

                // Check if folder exists, create it if not
                if !schema_path.exists() {
                    create_dir_all(schema_path)?;
                    // Create mod.rs file in the schema folder
                    let mod_file_path = schema_path.join(&module.common_file);
                    File::create(&mod_file_path)?;

                    // Write pub mod entry in root mod.rs
                    let schema_mod_entry = format!(
                        "{} {};\n",
                        &module.export_text,
                        schema.schema_name.to_case(Case::Snake)
                    );
                    root_mod_file.write_all(schema_mod_entry.as_bytes())?;
                }

                // Open mod.rs in append mode
                let mod_file_path = schema_path.join(&module.common_file);
                let mut mod_file = OpenOptions::new()
                    .write(true)
                    .append(true)
                    .open(&mod_file_path)?;

                for table in &schema.tables {
                    let table_file_name = format!(
                        "{}.{}",
                        table.table_name.to_case(Case::Snake),
                        &module.extension
                    );
                    let file_path = schema_path.join(table_file_name);

                    // Replace placeholders with actual values
                    let modified_contents =
                        self.replace_placeholders(&file_contents, &schema.schema_name, &table);

                    // Write to the new file
                    let mut new_file = File::create(file_path)?;
                    new_file.write_all(modified_contents.as_bytes())?;

                    let mod_entry = format!(
                        "{} {};\n",
                        &module.export_text,
                        table.table_name.to_case(Case::Snake)
                    );
                    mod_file.write_all(mod_entry.as_bytes())?;
                }
            }
        }

        Ok(())
    }

    fn replace_placeholders(&self, template: &str, schema: &str, table: &Table) -> String {
        let mut modified_template = template.to_string();

        for config in &self.config.column_configurations {
            let replacement = self.get_replacement_value(&config, schema, table);
            modified_template = modified_template.replace(&config.property_name, &replacement);
        }

        modified_template
    }

    fn get_replacement_value(
        &self,
        config: &ColumnConfiguration,
        schema: &str,
        table: &Table,
    ) -> String {
        let mut result = config.property_string.to_string();

        let convert_case = |text: &str, display_type: &str| -> String {
            if display_type == "rust" {
                text.to_case(Case::Snake)
            } else if display_type == "typescript" {
                text.to_case(Case::Camel)
            } else {
                text.to_string()
            }
        };

        let find_data_type = |data_type: &str, display_type: &str| -> String {
            for dt in &self.config.data_types {
                if dt.postgresql == data_type {
                    return match display_type {
                        "rust" => dt.rust.clone(),
                        "typescript" => dt.typescript.clone(),
                        _ => data_type.to_string(),
                    };
                }
            }
            data_type.to_string()
        };

        let formats: Vec<&str> = config.display_type.split('+').collect();
        let mut has_index = false;

        if result.contains("TABLE_NAME") && result.contains("SCHEMA") {
            result = result
                .replace("TABLE_NAME", &table.table_name)
                .replace("SCHEMA", schema);
        } else if result.contains("TABLE_NAME") {
            result = result.replace("TABLE_NAME", &table.table_name)
        } else if result.contains("PRIMARY_KEY") {
            result = result.replace(
                "PRIMARY_KEY",
                &convert_case(&table.columns[0].column_name, &config.display_type),
            );
        } else if result.contains("COLUMN_NAME") {
            result = "".to_string();

            for (index, col) in table.columns.iter().enumerate() {
                let mut column_text = config.property_string.to_string();

                if formats.len() == 2 {
                    column_text = column_text.replace(
                        "\"COLUMN_NAME\"",
                        &format!("\"{}\"", &convert_case(&col.column_name, formats[1])),
                    );

                    column_text = column_text
                        .replace("COLUMN_NAME", &convert_case(&col.column_name, formats[0]));
                }

                column_text = column_text.replace(
                    "COLUMN_NAME",
                    &convert_case(&col.column_name, &config.display_type),
                );
                if column_text.contains("DATA_TYPE") {
                    column_text = column_text.replace(
                        "DATA_TYPE",
                        &find_data_type(&col.data_type, &config.display_type),
                    );
                }

                if column_text.contains("COLUMN_INDEX") {
                    has_index = true;
                    column_text =
                        column_text.replace("COLUMN_INDEX", &col.ordinal_position.to_string());
                }

                if config.ends_with == ";" {
                    column_text = format!("{}{}", column_text, config.ends_with);
                } else if index < table.columns.len() - 1 {
                    column_text = format!("{}{}", column_text, config.ends_with);
                }

                if config.horizontal == "true" {
                    column_text.push('\n');
                }

                result = format!("{}{}", result, column_text);
            }

            if has_index == true {
                result = format!(
                    "{} WHERE {}",
                    result,
                    config
                        .property_string
                        .to_string()
                        .replace("COLUMN_NAME", &table.columns[0].column_name)
                        .replace("COLUMN_INDEX", "1")
                );
            }
        }

        result
    }
}
