use crate::data::types::{CsvSchema, ColumnConfig, DataType};
use anyhow::Result;

pub struct SchemaBuilder;

impl SchemaBuilder {
    pub fn build_schema(
        target_size: usize,
        num_rows: usize,
        min_columns: usize,
        max_columns: usize,
    ) -> Result<CsvSchema> {
        let target_row_size = Self::calculate_target_row_size(target_size, num_rows)?;
        let (_num_columns, column_sizes) = Self::optimize_column_distribution(
            target_row_size,
            min_columns,
            max_columns,
        )?;

        let columns = Self::create_columns(column_sizes)?;
        let header_size = Self::calculate_header_size(&columns);

        Ok(CsvSchema {
            columns,
            target_row_size,
            header_size,
        })
    }

    fn calculate_target_row_size(target_size: usize, num_rows: usize) -> Result<usize> {
        if num_rows == 0 {
            return Err(anyhow::anyhow!("Number of rows must be greater than 0"));
        }
        Ok(target_size / num_rows)
    }

    fn optimize_column_distribution(
        target_row_size: usize,
        min_columns: usize,
        max_columns: usize,
    ) -> Result<(usize, Vec<usize>)> {
        const COMMA_SIZE: usize = 1;
        const NEWLINE_SIZE: usize = 1;
        const MIN_COLUMN_DATA_SIZE: usize = 2;

        for num_columns in min_columns..=max_columns {
            let separator_overhead = (num_columns - 1) * COMMA_SIZE + NEWLINE_SIZE;
            let available_data_bytes = target_row_size.saturating_sub(separator_overhead);
            
            if available_data_bytes < num_columns * MIN_COLUMN_DATA_SIZE {
                continue;
            }

            let first_column_size = std::cmp::min(
                (available_data_bytes / 5).max(4),
                available_data_bytes / 2,
            );
            let remaining_bytes = available_data_bytes - first_column_size;
            let other_column_size = remaining_bytes / (num_columns - 1);

            let mut column_sizes = vec![first_column_size];
            column_sizes.extend(vec![other_column_size; num_columns - 1]);

            let actual_row_size = separator_overhead + column_sizes.iter().sum::<usize>();
            
            if (actual_row_size as f64 - target_row_size as f64).abs() < target_row_size as f64 * 0.1 {
                return Ok((num_columns, column_sizes));
            }
        }

        Err(anyhow::anyhow!("Could not find suitable column configuration"))
    }

    fn create_columns(column_sizes: Vec<usize>) -> Result<Vec<ColumnConfig>> {
        let mut columns = Vec::new();
        
        for (i, size) in column_sizes.iter().enumerate() {
            let name = if i == 0 {
                Self::generate_header_name("id", *size)
            } else {
                Self::generate_header_name(&format!("col{}", i), *size)
            };

            let data_type = if i == 0 {
                DataType::UniqueId
            } else {
                DataType::String
            };

            columns.push(ColumnConfig {
                name,
                size_bytes: *size,
                data_type,
            });
        }

        Ok(columns)
    }

    fn generate_header_name(base: &str, target_size: usize) -> String {
        if base.len() >= target_size {
            base.chars().take(target_size).collect()
        } else {
            let padding = target_size - base.len();
            format!("{}{}", base, "x".repeat(padding))
        }
    }

    fn calculate_header_size(columns: &[ColumnConfig]) -> usize {
        columns.iter().map(|c| c.name.len()).sum::<usize>() + (columns.len() - 1)
    }
}