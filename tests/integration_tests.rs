use csv_gen::{data::schema::SchemaBuilder, generator::size_based::CsvGenerator};
use std::fs;

#[test]
fn test_small_csv_generation() -> anyhow::Result<()> {
    let target_size = 100; // 100 bytes
    let num_rows = 5;
    
    let schema = SchemaBuilder::build_schema(target_size, num_rows, 2, 10)?;
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
    
    let schema = SchemaBuilder::build_schema(target_size, num_rows, min_columns, 10)?;
    
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
    
    let schema = SchemaBuilder::build_schema(target_size, num_rows, 2, 5)?;
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
    
    let schema = SchemaBuilder::build_schema(target_size, num_rows, 2, 5)?;
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
    
    let schema = SchemaBuilder::build_schema(target_size, num_rows, 2, 20)?;
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
    
    let schema = SchemaBuilder::build_schema(target_size, num_rows, 5, 15)?;
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