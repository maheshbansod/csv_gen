use clap::Parser;
use csv_gen::{cli::Args, data::schema::SchemaBuilder, generator::size_based::CsvGenerator, utils::file_ops};

fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    
    let target_size = args.parse_size()?;
    let num_rows = args.rows;
    
    println!("Generating CSV: {} with {} rows", args.size, num_rows);
    
    let schema = SchemaBuilder::build_schema(
        target_size,
        num_rows,
        args.min_columns,
        args.max_columns,
        args.email_columns,
        args.domain_columns,
    )?;
    
    println!("Schema: {} columns, target row size: {} bytes", 
             schema.columns.len(), 
             schema.target_row_size);
    
    file_ops::ensure_directory_exists(&args.output)?;
    
    let mut generator = CsvGenerator::new(schema);
    let rows_generated = generator.generate(&args.output, num_rows)?;
    
    let actual_size = file_ops::get_file_size(&args.output)?;
    println!("Generated {} rows in {} ({} bytes)", 
             rows_generated, 
             args.output, 
             actual_size);
    
    Ok(())
}
