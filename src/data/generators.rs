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
                // Smart email generation based on available space
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
                    // Medium to very large: proportional sizing
                    let (domain, local_part_size) = Self::select_domain_and_local_size(&mut self.rng, column.size_bytes);
                    let local = Self::generate_local_part(&mut self.rng, local_part_size);
                    let email = format!("{}@{}", local, domain);
                    
                    // Only truncate if absolutely necessary
                    if email.len() > column.size_bytes {
                        email.chars().take(column.size_bytes).collect()
                    } else {
                        email
                    }
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
                // Smart domain generation based on available space
                if column.size_bytes <= 5 {
                    // Very small: just use short domain
                    "a.co".chars().take(column.size_bytes).collect()
                } else if column.size_bytes <= 8 {
                    // Small: short domain with optional subdomain
                    let domains = ["a.co", "b.io", "c.dev", "d.app"];
                    let domain = domains[self.rng.gen_range(0..domains.len())];
                    domain.chars().take(column.size_bytes).collect()
                } else {
                    // Medium to very large: proportional domain generation
                    let domain = Self::generate_smart_domain(&mut self.rng, column.size_bytes);
                    if domain.len() > column.size_bytes {
                        domain.chars().take(column.size_bytes).collect()
                    } else {
                        domain
                    }
                }
            }
        }
    }

    fn select_domain_and_local_size(rng: &mut rand::rngs::ThreadRng, size_bytes: usize) -> (&'static str, usize) {
        match size_bytes {
            9..=25 => {
                // Medium: use medium domains, 60% for local part
                let domains = ["mail.com", "app.net", "web.org"];
                let domain = domains[rng.gen_range(0..domains.len())];
                let local_size = (size_bytes.saturating_sub(domain.len() + 1) as f64 * 0.6) as usize;
                (domain, local_size.max(3))
            }
            26..=50 => {
                // Large: use large domains, 70% for local part
                let domains = ["example.com", "company.org", "service.net"];
                let domain = domains[rng.gen_range(0..domains.len())];
                let local_size = (size_bytes.saturating_sub(domain.len() + 1) as f64 * 0.7) as usize;
                (domain, local_size.max(5))
            }
            51..=100 => {
                // Very large: use very large domains, 80% for local part
                let domains = ["corporation.io", "technology.com", "business.dev"];
                let domain = domains[rng.gen_range(0..domains.len())];
                let local_size = (size_bytes.saturating_sub(domain.len() + 1) as f64 * 0.8) as usize;
                (domain, local_size.max(8))
            }
            _ => {
                // Extra large: use extra large domains, 85% for local part
                let domains = ["enterprise.tech", "solutions.ai", "consulting.services"];
                let domain = domains[rng.gen_range(0..domains.len())];
                let local_size = (size_bytes.saturating_sub(domain.len() + 1) as f64 * 0.85) as usize;
                (domain, local_size.max(12))
            }
        }
    }

    fn generate_local_part(rng: &mut rand::rngs::ThreadRng, size: usize) -> String {
        if size <= 3 {
            // Very small: just letters
            (0..size)
                .map(|_| {
                    let chars = b"abcdefghijklmnopqrstuvwxyz";
                    chars[rng.gen_range(0..chars.len())] as char
                })
                .collect()
        } else if size <= 8 {
            // Small: letters + numbers
            (0..size)
                .map(|_| {
                    if rng.gen_range(0..2) == 0 {
                        let chars = b"abcdefghijklmnopqrstuvwxyz";
                        chars[rng.gen_range(0..chars.len())] as char
                    } else {
                        let digits = b"0123456789";
                        digits[rng.gen_range(0..digits.len())] as char
                    }
                })
                .collect()
        } else if size <= 20 {
            // Medium: name patterns
            let first_names = ["john", "jane", "bob", "alice", "tom", "sue"];
            let last_names = ["smith", "doe", "johnson", "brown", "davis"];
            let first = first_names[rng.gen_range(0..first_names.len())];
            let last = last_names[rng.gen_range(0..last_names.len())];
            
            let base = format!("{}.{}", first, last);
            if base.len() >= size {
                base.chars().take(size).collect()
            } else {
                let remaining = size - base.len();
                let suffix: String = (0..remaining)
                    .map(|_| {
                        let digits = b"0123456789";
                        digits[rng.gen_range(0..digits.len())] as char
                    })
                    .collect();
                format!("{}{}", base, suffix)
            }
        } else {
            // Large: complex patterns
            let first_names = ["john", "jane", "bob", "alice", "tom", "sue"];
            let last_names = ["smith", "doe", "johnson", "brown", "davis"];
            let middle_names = ["william", "james", "robert", "michael", "david"];
            let suffixes = ["", "jr", "sr", "ii", "iii"];
            
            let first = first_names[rng.gen_range(0..first_names.len())];
            let middle = middle_names[rng.gen_range(0..middle_names.len())];
            let last = last_names[rng.gen_range(0..last_names.len())];
            let suffix = suffixes[rng.gen_range(0..suffixes.len())];
            
            let base = if suffix.is_empty() {
                format!("{}.{}.{}", first, middle, last)
            } else {
                format!("{}.{}.{}.{}", first, middle, last, suffix)
            };
            
            if base.len() >= size {
                base.chars().take(size).collect()
            } else {
                let remaining = size - base.len();
                let suffix_num: String = (0..remaining)
                    .map(|_| {
                        let digits = b"0123456789";
                        digits[rng.gen_range(0..digits.len())] as char
                    })
                    .collect();
                format!("{}{}", base, suffix_num)
            }
        }
    }

    fn generate_smart_domain(rng: &mut rand::rngs::ThreadRng, size_bytes: usize) -> String {
        match size_bytes {
            9..=20 => {
                // Medium: simple domains
                let domains = ["mail.com", "app.net", "web.org", "site.io"];
                let domain = domains[rng.gen_range(0..domains.len())];
                if rng.gen_range(0..3) == 0 {
                    // Add subdomain occasionally
                    let subdomains = ["www.", "api.", "app."];
                    let subdomain = subdomains[rng.gen_range(0..subdomains.len())];
                    format!("{}{}", subdomain, domain)
                } else {
                    domain.to_string()
                }
            }
            21..=40 => {
                // Large: more complex domains
                let domains = ["example.com", "company.org", "service.net", "platform.dev"];
                let subdomains = ["", "www.", "api.", "app.", "admin.", "blog."];
                let domain = domains[rng.gen_range(0..domains.len())];
                let subdomain = subdomains[rng.gen_range(0..subdomains.len())];
                format!("{}{}", subdomain, domain)
            }
            41..=80 => {
                // Very large: corporate domains
                let domains = ["corporation.com", "technology.org", "business.net", "enterprise.dev"];
                let subdomains = ["", "www.", "api.", "app.", "admin.", "blog.", "shop.", "mail.", "secure."];
                let domain = domains[rng.gen_range(0..domains.len())];
                let subdomain = subdomains[rng.gen_range(0..subdomains.len())];
                format!("{}{}", subdomain, domain)
            }
            _ => {
                // Extra large: complex domains
                let domains = ["consulting.services", "solutions.technology", "enterprise.corporation", "innovation.platform"];
                let subdomains = ["", "www.", "api.", "app.", "admin.", "blog.", "shop.", "mail.", "secure.", "internal.", "external."];
                let domain = domains[rng.gen_range(0..domains.len())];
                let subdomain = subdomains[rng.gen_range(0..subdomains.len())];
                format!("{}{}", subdomain, domain)
            }
        }
    }
}