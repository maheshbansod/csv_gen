use crate::data::{types::CsvSchema, generators::DataGenerator};
use anyhow::Result;
use csv::WriterBuilder;
use indicatif::{ProgressBar, ProgressStyle};
use std::fs::File;

pub struct CsvGenerator {
    schema: CsvSchema,
    data_generator: DataGenerator,
}

impl CsvGenerator {
    pub fn new(schema: CsvSchema) -> Self {
        Self {
            schema,
            data_generator: DataGenerator::new(),
        }
    }

    pub fn generate(&mut self, output_path: &str, num_rows: usize) -> Result<usize> {
        let file = File::create(output_path)?;
        let mut writer = WriterBuilder::new().from_writer(file);

        let pb = ProgressBar::new(num_rows as u64);
        pb.set_style(
            ProgressStyle::default_bar()
                .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({eta})")
                .unwrap()
                .progress_chars("#>-"),
        );

        let header: Vec<String> = self.schema.columns.iter().map(|c| c.name.clone()).collect();
        writer.write_record(&header)?;

        for _ in 0..num_rows {
            let row: Vec<String> = self.schema
                .columns
                .iter()
                .map(|col| self.data_generator.generate_value(col))
                .collect();
            
            writer.write_record(&row)?;
            pb.inc(1);
        }

        writer.flush()?;
        pb.finish_with_message("CSV generation complete!");

        Ok(num_rows)
    }

    pub fn get_estimated_size(&self, num_rows: usize) -> usize {
        self.schema.header_size + (num_rows * self.schema.target_row_size)
    }
}