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
        // Use iterative approach to account for header size
        let (columns, target_row_size, header_size) = Self::build_schema_iterative(
            target_size,
            num_rows,
            min_columns,
            max_columns,
        )?;

        Ok(CsvSchema {
            columns,
            target_row_size,
            header_size,
        })
    }

    fn build_schema_iterative(
        target_size: usize,
        num_rows: usize,
        min_columns: usize,
        max_columns: usize,
    ) -> Result<(Vec<crate::data::types::ColumnConfig>, usize, usize)> {
        let mut best_result = None;
        let mut best_error = usize::MAX;
        
        // Try different column counts to find the best fit
        for num_columns in min_columns..=max_columns {
            // Calculate what row size we need to hit the target
            let estimated_header_size = Self::estimate_header_for_columns(num_columns);
            let available_for_data = target_size.saturating_sub(estimated_header_size);
            
            if available_for_data == 0 || num_rows == 0 {
                continue;
            }
            
            let target_row_size = available_for_data / num_rows;
            
            // Try to create a schema with this column count
            match Self::create_schema_for_exact_columns(num_columns, target_row_size) {
                Ok((columns, actual_row_size)) => {
                    let actual_header_size = Self::calculate_header_size(&columns);
                    let total_size = actual_header_size + (num_rows * actual_row_size);
                    
                    let size_error = if total_size > target_size {
                        total_size - target_size
                    } else {
                        target_size - total_size
                    };
                    
                    // Update best result if this is closer to target
                    if size_error < best_error {
                        best_error = size_error;
                        best_result = Some((columns, actual_row_size, actual_header_size));
                        
                        // If we're very close (within 0.1%), we can stop
                        if size_error < target_size / 1000 {
                            break;
                        }
                    }
                }
                Err(_) => continue,
            }
        }
        
        best_result.ok_or_else(|| anyhow::anyhow!("Could not find suitable column configuration"))
    }

    fn estimate_header_for_columns(num_columns: usize) -> usize {
        // Estimate header size based on column count
        let avg_column_name_length = 8; // Average like "col1xxxx"
        num_columns * avg_column_name_length + (num_columns - 1)
    }

    fn create_schema_for_exact_columns(
        num_columns: usize,
        target_row_size: usize,
    ) -> Result<(Vec<crate::data::types::ColumnConfig>, usize)> {
        const COMMA_SIZE: usize = 1;
        const NEWLINE_SIZE: usize = 1;
        const MIN_COLUMN_DATA_SIZE: usize = 2;
        
        let separator_overhead = (num_columns - 1) * COMMA_SIZE + NEWLINE_SIZE;
        let available_data_bytes = target_row_size.saturating_sub(separator_overhead);
        
        if available_data_bytes < num_columns * MIN_COLUMN_DATA_SIZE {
            return Err(anyhow::anyhow!("Not enough space for {} columns", num_columns));
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
        let columns = Self::create_columns(column_sizes)?;
        
        Ok((columns, actual_row_size))
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