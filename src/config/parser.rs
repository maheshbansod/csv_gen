use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneratorConfig {
    pub default_min_columns: usize,
    pub default_max_columns: usize,
    pub min_column_data_size: usize,
    pub max_column_data_size: usize,
}

impl Default for GeneratorConfig {
    fn default() -> Self {
        Self {
            default_min_columns: 2,
            default_max_columns: 100,
            min_column_data_size: 2,
            max_column_data_size: 50,
        }
    }
}

impl GeneratorConfig {
    pub fn load_from_file(path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let content = fs::read_to_string(path)?;
        let config: GeneratorConfig = serde_json::from_str(&content)?;
        Ok(config)
    }

    pub fn save_to_file(&self, path: &str) -> Result<(), Box<dyn std::error::Error>> {
        let content = serde_json::to_string_pretty(self)?;
        fs::write(path, content)?;
        Ok(())
    }
}