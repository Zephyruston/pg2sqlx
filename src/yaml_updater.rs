//! YAML configuration updater module
//! This module is responsible for updating goctl.yaml with custom type mappings

use std::collections::HashMap;
use std::fs;
use std::io::{Read, Write};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TypeMapping {
    #[serde(rename = "null_type")]
    pub null_type: String,
    #[serde(rename = "type")]
    pub type_name: String,
    #[serde(rename = "pkg", skip_serializing_if = "Option::is_none")]
    pub pkg: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ModelConfig {
    #[serde(rename = "types_map")]
    pub types_map: HashMap<String, TypeMapping>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Config {
    pub model: ModelConfig,
}

pub struct YamlUpdater {
    config: Config,
}

impl YamlUpdater {
    pub fn new(config_path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let mut file = fs::File::open(config_path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        
        let config: Config = serde_yaml::from_str(&contents)?;
        Ok(YamlUpdater { config })
    }

    pub fn add_enum_mapping(&mut self, enum_name: &str) {
        self.config.model.types_map.insert(
            enum_name.to_string(),
            TypeMapping {
                null_type: "sql.NullString".to_string(),
                type_name: "string".to_string(),
                pkg: None,
            },
        );
    }

    pub fn add_vector_mapping(&mut self) {
        self.config.model.types_map.insert(
            "vector".to_string(),
            TypeMapping {
                null_type: "sql.NullString".to_string(),
                type_name: "string".to_string(),
                pkg: Some("github.com/pgvector/pgvector-go".to_string()),
            },
        );
    }

    pub fn save_to_file(&self, file_path: &str) -> Result<(), Box<dyn std::error::Error>> {
        // Create backup
        if fs::metadata(file_path).is_ok() {
            fs::copy(file_path, format!("{}.bak", file_path))?;
        }
        
        let yaml_string = serde_yaml::to_string(&self.config)?;
        let mut file = fs::File::create(file_path)?;
        file.write_all(yaml_string.as_bytes())?;
        Ok(())
    }
    
    pub fn save_to_new_file(&self, file_path: &str) -> Result<(), Box<dyn std::error::Error>> {
        let yaml_string = serde_yaml::to_string(&self.config)?;
        let mut file = fs::File::create(file_path)?;
        file.write_all(yaml_string.as_bytes())?;
        Ok(())
    }
    
    /// Get the types map as a sorted vector
    pub fn get_sorted_types_map(&self) -> Vec<(&String, &TypeMapping)> {
        let mut types: Vec<(&String, &TypeMapping)> = self.config.model.types_map.iter().collect();
        types.sort_by(|a, b| a.0.cmp(b.0));
        types
    }
    
    /// Get a reference to the config
    pub fn config(&self) -> &Config {
        &self.config
    }
    
    /// Create a new YamlUpdater with a given config
    pub fn with_config(config: Config) -> Self {
        YamlUpdater { config }
    }
}