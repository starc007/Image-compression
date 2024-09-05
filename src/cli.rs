
//! Command-line interface parsing module.

use clap::Parser;

/// Struct to hold command-line arguments
#[derive(Parser)]
#[clap(version = "1.0", author = "Your Name", about = "Compresses images in a folder")]
pub struct Opts {
    /// Input directory path
    #[clap(short, long, default_value = ".")]
    pub input: String,
    
    /// Output directory path
    #[clap(short, long, default_value = "compressed_images")]
    pub output: String,
    
    /// Compression quality (0-100)
    #[clap(short, long, default_value = "85")]
    pub quality: u8,
}

/// Parse command-line arguments
pub fn parse_args() -> Opts {
    Opts::parse()
}