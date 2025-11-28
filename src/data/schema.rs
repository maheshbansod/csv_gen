use crate::data::types::{CsvSchema, ColumnConfig, DataType};
use anyhow::Result;

pub struct SchemaBuilder;

impl SchemaBuilder {
    pub fn build_schema(
        target_size: usize,
        num_rows: usize,
        min_columns: usize,
        max_columns: usize,
        email_columns: usize,
        domain_columns: usize,
    ) -> Result<CsvSchema> {
        // Use iterative approach to account for header size
        let (columns, target_row_size, header_size) = Self::build_schema_iterative(
            target_size,
            num_rows,
            min_columns,
            max_columns,
            email_columns,
            domain_columns,
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
        email_columns: usize,
        domain_columns: usize,
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
            match Self::create_schema_for_exact_columns(num_columns, target_row_size, email_columns, domain_columns) {
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
        email_columns: usize,
        domain_columns: usize,
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
        let columns = Self::create_columns(column_sizes, email_columns, domain_columns)?;
        
        Ok((columns, actual_row_size))
    }



    fn create_columns(column_sizes: Vec<usize>, email_columns: usize, domain_columns: usize) -> Result<Vec<ColumnConfig>> {
        let mut columns = Vec::new();
        let mut email_count = 0;
        let mut domain_count = 0;
        
        for (i, size) in column_sizes.iter().enumerate() {
            let (name, data_type) = if i == 0 {
                (Self::generate_header_name("id", *size), DataType::UniqueId)
            } else if email_count < email_columns {
                email_count += 1;
                (Self::generate_header_name(&format!("email_{}", email_count), *size), DataType::Email)
            } else if domain_count < domain_columns {
                domain_count += 1;
                (Self::generate_header_name(&format!("domain_{}", domain_count), *size), DataType::Domain)
            } else {
                (Self::generate_header_name(&format!("col{}", i), *size), DataType::String)
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
        // For very small columns, use unique single characters
        if target_size <= 2 {
            match base {
                "id" => "id".chars().take(target_size).collect(),
                _ => {
                    // Extract column number for unique single character
                    if let Some(num_str) = base.strip_prefix("col") {
                        let num = num_str.parse::<usize>().unwrap_or(0);
                        let unique_char = match num % 26 {
                            0 => 'a', 1 => 'b', 2 => 'c', 3 => 'd', 4 => 'e',
                            5 => 'f', 6 => 'g', 7 => 'h', 8 => 'i', 9 => 'j',
                            10 => 'k', 11 => 'l', 12 => 'm', 13 => 'n', 14 => 'o',
                            15 => 'p', 16 => 'q', 17 => 'r', 18 => 's', 19 => 't',
                            20 => 'u', 21 => 'v', 22 => 'w', 23 => 'x', 24 => 'y',
                            _ => 'z'
                        };
                        unique_char.to_string().chars().take(target_size).collect()
                    } else {
                        base.chars().take(target_size).collect()
                    }
                }
            }
        } else {
            // Normal case: ensure minimum size for uniqueness
            let min_size = std::cmp::max(target_size, 4);
            
            if base.len() >= min_size {
                base.chars().take(min_size).collect()
            } else {
                let padding = min_size - base.len();
                // Use different padding characters to ensure uniqueness
                let padding_char = match base.chars().next() {
                    Some('i') => 'x',  // id -> xxx
                    Some('c') => {
                        // Extract column number for unique padding
                        if let Some(num_str) = base.strip_prefix("col") {
                            let num = num_str.parse::<usize>().unwrap_or(0);
                            match num % 10 {
                                0 => 'a', 1 => 'b', 2 => 'c', 3 => 'd', 4 => 'e',
                                5 => 'f', 6 => 'g', 7 => 'h', 8 => 'i', 9 => 'j',
                                _ => 'x'
                            }
                        } else {
                            'x'
                        }
                    }
                    _ => 'x',
                };
                format!("{}{}", base, padding_char.to_string().repeat(padding))
            }
        }
    }

    fn calculate_header_size(columns: &[ColumnConfig]) -> usize {
        columns.iter().map(|c| c.name.len()).sum::<usize>() + (columns.len() - 1)
    }
}