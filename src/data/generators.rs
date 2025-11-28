use rand::Rng;
use rand::thread_rng;
use crate::data::types::{ColumnConfig, DataType};

pub struct DataGenerator {
    rng: rand::rngs::ThreadRng,
    id_counter: usize,
}

impl DataGenerator {
    pub fn new() -> Self {
        Self {
            rng: thread_rng(),
            id_counter: 1,
        }
    }

    pub fn generate_value(&mut self, column: &ColumnConfig) -> String {
        match &column.data_type {
            DataType::UniqueId => {
                let id = self.id_counter;
                self.id_counter += 1;
                format!("{:0width$}", id, width = column.size_bytes)
            }
            DataType::String => {
                let chars: Vec<char> = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789"
                    .chars()
                    .collect();
                (0..column.size_bytes)
                    .map(|_| chars[self.rng.gen_range(0..chars.len())])
                    .collect()
            }
            DataType::Number => {
                let max = 10u64.pow(column.size_bytes as u32);
                self.rng.gen_range(0..max).to_string()
            }
            DataType::Email => {
                let domains = vec!["test.com", "example.org", "demo.net"];
                let domain = domains[self.rng.gen_range(0..domains.len())];
                let local_len = column.size_bytes.saturating_sub(domain.len() + 1);
                let local: String = (0..local_len)
                    .map(|_| self.rng.gen_range(b'a'..=b'z') as char)
                    .collect();
                format!("{}@{}", local, domain)
            }
            DataType::Name => {
                let first_names = vec!["John", "Jane", "Bob", "Alice", "Tom", "Sue"];
                let last_names = vec!["Smith", "Doe", "Johnson", "Brown", "Davis"];
                let first = first_names[self.rng.gen_range(0..first_names.len())];
                let last = last_names[self.rng.gen_range(0..last_names.len())];
                let name = format!("{} {}", first, last);
                if name.len() > column.size_bytes {
                    name.chars().take(column.size_bytes).collect()
                } else {
                    name
                }
            }
        }
    }
}