#[cfg(test)]
mod tests {
    use csv_gen::data::schema::SchemaBuilder;
    use csv_gen::generator::size_based::CsvGenerator;
    use std::collections::HashSet;
    use std::fs;

    #[test]
    fn test_no_duplicate_headers_many_columns() -> anyhow::Result<()> {
        // Test with many columns to stress test the uniqueness logic
        let target_size = 2000;
        let num_rows = 10;
        
        let schema = SchemaBuilder::build_schema(target_size, num_rows, 50, 100, 10, 10)?;
        let mut generator = CsvGenerator::new(schema);
        
        let output_path = "test_no_duplicates_many_columns.csv";
        generator.generate(output_path, num_rows)?;
        
        let content = fs::read_to_string(output_path)?;
        let header_line = content.lines().next().unwrap();
        let headers: Vec<&str> = header_line.split(',').collect();
        
        // Check that all headers are unique
        let mut unique_headers = HashSet::new();
        for header in &headers {
            assert!(!unique_headers.contains(*header), "Duplicate header found: {}", header);
            unique_headers.insert(*header);
        }
        
        // Verify we have a reasonable number of columns
        assert!(headers.len() >= 50);
        
        fs::remove_file(output_path)?;
        Ok(())
    }

    

    #[test]
    fn test_no_duplicate_headers_mixed_types() -> anyhow::Result<()> {
        // Test with mixed column types where similar base names could cause conflicts
        let target_size = 800;
        let num_rows = 8;
        
        let schema = SchemaBuilder::build_schema(target_size, num_rows, 15, 25, 5, 5)?;
        let mut generator = CsvGenerator::new(schema);
        
        let output_path = "test_no_duplicates_mixed_types.csv";
        generator.generate(output_path, num_rows)?;
        
        let content = fs::read_to_string(output_path)?;
        let header_line = content.lines().next().unwrap();
        let headers: Vec<&str> = header_line.split(',').collect();
        
        // Check that all headers are unique
        let mut unique_headers = HashSet::new();
        for header in &headers {
            assert!(!unique_headers.contains(*header), "Duplicate header found: {}", header);
            unique_headers.insert(*header);
        }
        
        // Count different header types
        let email_headers: Vec<_> = headers.iter().filter(|h| h.starts_with("email_")).collect();
        let domain_headers: Vec<_> = headers.iter().filter(|h| h.starts_with("domain_")).collect();
        let id_headers: Vec<_> = headers.iter().filter(|h| h.starts_with("id")).collect();
        
        assert_eq!(email_headers.len(), 5);
        assert_eq!(domain_headers.len(), 5);
        assert_eq!(id_headers.len(), 1);
        
        fs::remove_file(output_path)?;
        Ok(())
    }
}