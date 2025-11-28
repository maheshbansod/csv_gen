use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ColumnConfig {
    pub name: String,
    pub size_bytes: usize,
    pub data_type: DataType,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DataType {
    UniqueId,
    String,
    Number,
    Email,
    Name,
}

#[derive(Debug, Clone)]
pub struct CsvSchema {
    pub columns: Vec<ColumnConfig>,
    pub target_row_size: usize,
    pub header_size: usize,
}

impl CsvSchema {
    pub fn calculate_header_size(&self) -> usize {
        self.columns
            .iter()
            .map(|c| c.name.len())
            .sum::<usize>() + (self.columns.len() - 1)
    }
}