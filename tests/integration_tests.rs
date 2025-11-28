use csv_gen::{data::schema::SchemaBuilder, generator::size_based::CsvGenerator};
use std::fs;

#[test]
fn test_small_csv_generation() -> anyhow::Result<()> {
    let target_size = 100; // 100 bytes
    let num_rows = 5;
    
    let schema = SchemaBuilder::build_schema(target_size, num_rows, 2, 10, 0, 0)?;
    let mut generator = CsvGenerator::new(schema);
    
    let output_path = "test_small.csv";
    let rows_generated = generator.generate(output_path, num_rows)?;
    
    assert_eq!(rows_generated, num_rows);
    
    let content = fs::read_to_string(output_path)?;
    let lines: Vec<&str> = content.trim().split('\n').collect();
    assert_eq!(lines.len(), num_rows + 1); // +1 for header
    
    // Check that first column has unique IDs
    let mut ids = std::collections::HashSet::new();
    for line in lines.iter().skip(1) {
        let id = line.split(',').next().unwrap();
        ids.insert(id);
    }
    assert_eq!(ids.len(), num_rows);
    
    fs::remove_file(output_path)?;
    Ok(())
}

#[test]
fn test_minimum_columns() -> anyhow::Result<()> {
    let target_size = 50;
    let num_rows = 3;
    let min_columns = 2;
    
    let schema = SchemaBuilder::build_schema(target_size, num_rows, min_columns, 10, 0, 0)?;
    
    assert!(schema.columns.len() >= min_columns);
    
    let mut generator = CsvGenerator::new(schema);
    let output_path = "test_min_columns.csv";
    generator.generate(output_path, num_rows)?;
    
    let content = fs::read_to_string(output_path)?;
    let header_line = content.lines().next().unwrap();
    let column_count = header_line.split(',').count();
    
    assert!(column_count >= min_columns);
    
    fs::remove_file(output_path)?;
    Ok(())
}

#[test]
fn test_single_row_generation() -> anyhow::Result<()> {
    let target_size = 30;
    let num_rows = 1;
    
    let schema = SchemaBuilder::build_schema(target_size, num_rows, 2, 5, 0, 0)?;
    let mut generator = CsvGenerator::new(schema);
    
    let output_path = "test_single_row.csv";
    let rows_generated = generator.generate(output_path, num_rows)?;
    
    assert_eq!(rows_generated, 1);
    
    let content = fs::read_to_string(output_path)?;
    let lines: Vec<&str> = content.trim().split('\n').collect();
    assert_eq!(lines.len(), 2); // 1 header + 1 data row
    
    fs::remove_file(output_path)?;
    Ok(())
}

#[test]
fn test_unique_id_generation() -> anyhow::Result<()> {
    let target_size = 200;
    let num_rows = 10;
    
    let schema = SchemaBuilder::build_schema(target_size, num_rows, 2, 5, 0, 0)?;
    let mut generator = CsvGenerator::new(schema);
    
    let output_path = "test_unique_ids.csv";
    generator.generate(output_path, num_rows)?;
    
    let content = fs::read_to_string(output_path)?;
    let lines: Vec<&str> = content.trim().split('\n').collect();
    
    let mut ids = Vec::new();
    for line in lines.iter().skip(1) {
        let id = line.split(',').next().unwrap();
        ids.push(id.parse::<usize>().unwrap());
    }
    
    // Check that IDs are sequential and unique
    for (i, &id) in ids.iter().enumerate() {
        assert_eq!(id, i + 1);
    }
    
    fs::remove_file(output_path)?;
    Ok(())
}

#[test]
fn test_size_approximation() -> anyhow::Result<()> {
    let target_size = 500;
    let num_rows = 10;
    
    let schema = SchemaBuilder::build_schema(target_size, num_rows, 2, 20, 0, 0)?;
    let mut generator = CsvGenerator::new(schema);
    
    let output_path = "test_size_approx.csv";
    generator.generate(output_path, num_rows)?;
    
    let actual_size = fs::metadata(output_path)?.len() as usize;
    
    // Allow 20% tolerance for size approximation
    let tolerance = target_size as f64 * 0.2;
    assert!((actual_size as f64 - target_size as f64).abs() <= tolerance);
    
    fs::remove_file(output_path)?;
    Ok(())
}

#[test]
fn test_exact_size_targeting() -> anyhow::Result<()> {
    let target_size = 1024; // 1KB
    let num_rows = 10;
    
    let schema = SchemaBuilder::build_schema(target_size, num_rows, 5, 15, 0, 0)?;
    let mut generator = CsvGenerator::new(schema);
    
    let output_path = "test_exact_size.csv";
    generator.generate(output_path, num_rows)?;
    
    let actual_size = fs::metadata(output_path)?.len() as usize;
    
    // Should be within 2% of target size
    let tolerance = target_size as f64 * 0.02;
    assert!((actual_size as f64 - target_size as f64).abs() <= tolerance);
    
    fs::remove_file(output_path)?;
    Ok(())
}

#[test]
fn test_unique_headers() -> anyhow::Result<()> {
    let target_size = 500;
    let num_rows = 5;
    
    let schema = SchemaBuilder::build_schema(target_size, num_rows, 8, 12, 0, 0)?;
    let mut generator = CsvGenerator::new(schema);
    
    let output_path = "test_unique_headers.csv";
    generator.generate(output_path, num_rows)?;
    
    let content = fs::read_to_string(output_path)?;
    let header_line = content.lines().next().unwrap();
    let headers: Vec<&str> = header_line.split(',').collect();
    
    // Check that all headers are unique
    let mut unique_headers = std::collections::HashSet::new();
    for header in &headers {
        assert!(!unique_headers.contains(*header), "Duplicate header found: {}", header);
        unique_headers.insert(*header);
    }
    
    // Check that we have the expected number of columns
    assert!(headers.len() >= 8);
    
    fs::remove_file(output_path)?;
    Ok(())
}

#[test]
fn test_email_columns() -> anyhow::Result<()> {
    let target_size = 500;
    let num_rows = 5;
    let email_columns = 2;
    
    let schema = SchemaBuilder::build_schema(target_size, num_rows, 3, 10, email_columns, 0)?;
    let mut generator = CsvGenerator::new(schema);
    
    let output_path = "test_email_columns.csv";
    generator.generate(output_path, num_rows)?;
    
    let content = fs::read_to_string(output_path)?;
    let header_line = content.lines().next().unwrap();
    let headers: Vec<&str> = header_line.split(',').collect();
    
    // Check that we have email columns with correct prefixes
    let email_headers: Vec<&str> = headers.iter().filter(|h| h.starts_with("email_")).copied().collect();
    assert_eq!(email_headers.len(), email_columns);
    
    // Check that email columns contain valid email format
    let data_lines: Vec<&str> = content.lines().skip(1).collect();
    for line in data_lines {
        let columns: Vec<&str> = line.split(',').collect();
        for (i, header) in headers.iter().enumerate() {
            if header.starts_with("email_") {
                let value = columns[i];
                assert!(value.contains('@'), "Email column should contain @: {}", value);
            }
        }
    }
    
    fs::remove_file(output_path)?;
    Ok(())
}

#[test]
fn test_domain_columns() -> anyhow::Result<()> {
    let target_size = 400;
    let num_rows = 5;
    let domain_columns = 2;
    
    let schema = SchemaBuilder::build_schema(target_size, num_rows, 3, 10, 0, domain_columns)?;
    let mut generator = CsvGenerator::new(schema);
    
    let output_path = "test_domain_columns.csv";
    generator.generate(output_path, num_rows)?;
    
    let content = fs::read_to_string(output_path)?;
    let header_line = content.lines().next().unwrap();
    let headers: Vec<&str> = header_line.split(',').collect();
    
    // Check that we have domain columns with correct prefixes
    let domain_headers: Vec<&str> = headers.iter().filter(|h| h.starts_with("domain_")).copied().collect();
    assert_eq!(domain_headers.len(), domain_columns);
    
    // Check that domain columns contain valid domain format
    let data_lines: Vec<&str> = content.lines().skip(1).collect();
    for line in data_lines {
        let columns: Vec<&str> = line.split(',').collect();
        for (i, header) in headers.iter().enumerate() {
            if header.starts_with("domain_") {
                let value = columns[i];
                assert!(value.contains('.'), "Domain column should contain .: {}", value);
                assert!(!value.contains('@'), "Domain column should not contain @: {}", value);
            }
        }
    }
    
    fs::remove_file(output_path)?;
    Ok(())
}

#[test]
fn test_mixed_column_types() -> anyhow::Result<()> {
    let target_size = 600;
    let num_rows = 5;
    let email_columns = 2;
    let domain_columns = 1;
    
    let schema = SchemaBuilder::build_schema(target_size, num_rows, 5, 15, email_columns, domain_columns)?;
    let mut generator = CsvGenerator::new(schema);
    
    let output_path = "test_mixed_columns.csv";
    generator.generate(output_path, num_rows)?;
    
    let content = fs::read_to_string(output_path)?;
    let header_line = content.lines().next().unwrap();
    let headers: Vec<&str> = header_line.split(',').collect();
    
    // Check that we have the right number of each column type
    let email_headers: Vec<&str> = headers.iter().filter(|h| h.starts_with("email_")).copied().collect();
    let domain_headers: Vec<&str> = headers.iter().filter(|h| h.starts_with("domain_")).copied().collect();
    
    assert_eq!(email_headers.len(), email_columns);
    assert_eq!(domain_headers.len(), domain_columns);
    
    // Check that all headers are unique
    let mut unique_headers = std::collections::HashSet::new();
    for header in &headers {
        assert!(!unique_headers.contains(*header), "Duplicate header found: {}", header);
        unique_headers.insert(*header);
    }
    
    fs::remove_file(output_path)?;
    Ok(())
}