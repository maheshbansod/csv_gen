use std::fs;
use std::path::Path;
use anyhow::Result;

pub fn ensure_directory_exists(file_path: &str) -> Result<()> {
    if let Some(parent) = Path::new(file_path).parent() {
        fs::create_dir_all(parent)?;
    }
    Ok(())
}

pub fn get_file_size(file_path: &str) -> Result<usize> {
    let metadata = fs::metadata(file_path)?;
    Ok(metadata.len() as usize)
}