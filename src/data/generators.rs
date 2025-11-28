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
                // Generate email that fits exactly in column size
                if column.size_bytes <= 5 {
                    // Very small: just use single letter @ short domain
                    "a@b.co".chars().take(column.size_bytes).collect()
                } else if column.size_bytes <= 8 {
                    // Small: 2-3 chars @ short domain
                    let domains = ["a.co", "b.io", "c.dev"];
                    let domain = domains[self.rng.gen_range(0..domains.len())];
                    let local_len = column.size_bytes.saturating_sub(domain.len() + 1);
                    let local: String = (0..local_len)
                        .map(|_| self.rng.gen_range(b'a'..=b'z') as char)
                        .collect();
                    format!("{}@{}", local, domain)
                } else {
                    // Larger: normal email generation
                    let domains = ["test.com", "example.org", "demo.net"];
                    let domain = domains[self.rng.gen_range(0..domains.len())];
                    let local_len = std::cmp::min(column.size_bytes.saturating_sub(domain.len() + 1), 10);
                    let local: String = (0..local_len)
                        .map(|_| self.rng.gen_range(b'a'..=b'z') as char)
                        .collect();
                    let email = format!("{}@{}", local, domain);
                    email.chars().take(column.size_bytes).collect()
                }
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
            DataType::Domain => {
                // Generate domain that fits exactly in column size
                if column.size_bytes <= 5 {
                    // Very small: just use short domain
                    "a.co".chars().take(column.size_bytes).collect()
                } else if column.size_bytes <= 8 {
                    // Small: short domain with optional subdomain
                    let domains = ["a.co", "b.io", "c.dev", "d.app"];
                    let domain = domains[self.rng.gen_range(0..domains.len())];
                    domain.chars().take(column.size_bytes).collect()
                } else {
                    // Larger: normal domain generation
                    let domains = ["example.com", "test.org", "demo.net"];
                    let subdomains = ["", "www.", "api.", "app."];
                    let domain = domains[self.rng.gen_range(0..domains.len())];
                    let subdomain = subdomains[self.rng.gen_range(0..subdomains.len())];
                    let full_domain = format!("{}{}", subdomain, domain);
                    full_domain.chars().take(column.size_bytes).collect()
                }
            }
        }
    }
}