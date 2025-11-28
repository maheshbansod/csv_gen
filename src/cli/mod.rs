use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "csvgen")]
#[command(about = "A scalable CSV generator with size and row control")]
pub struct Args {
    /// Target file size (e.g., 1MB, 500KB, 2GB)
    #[arg(short, long)]
    pub size: String,

    /// Number of rows to generate
    #[arg(short, long)]
    pub rows: usize,

    /// Output file path
    #[arg(short, long, default_value = "output.csv")]
    pub output: String,

    /// Minimum number of columns
    #[arg(long, default_value = "2")]
    pub min_columns: usize,

    /// Maximum number of columns
    #[arg(long, default_value = "100")]
    pub max_columns: usize,
}

impl Args {
    pub fn parse_size(&self) -> Result<usize, anyhow::Error> {
        let size_str = self.size.to_uppercase();
        if size_str.ends_with("KB") {
            let num: f64 = size_str.trim_end_matches("KB").parse()?;
            Ok((num * 1024.0) as usize)
        } else if size_str.ends_with("MB") {
            let num: f64 = size_str.trim_end_matches("MB").parse()?;
            Ok((num * 1024.0 * 1024.0) as usize)
        } else if size_str.ends_with("GB") {
            let num: f64 = size_str.trim_end_matches("GB").parse()?;
            Ok((num * 1024.0 * 1024.0 * 1024.0) as usize)
        } else if size_str.ends_with("B") {
            let num: f64 = size_str.trim_end_matches("B").parse()?;
            Ok(num as usize)
        } else {
            Err(anyhow::anyhow!("Invalid size format. Use KB, MB, GB, or B"))
        }
    }
}