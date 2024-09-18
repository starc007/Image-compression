use clap::Parser;

/**
 * Struct to hold command-line arguments
 */
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

     /// Maximum image dimension
    #[clap(long, default_value = "1280")]
    pub max_dimension: u32,
}

/**
 * Parse command-line arguments
 * 
 * This function uses the clap crate to parse command-line arguments
 * based on the Opts struct definition.
 * 
 * # Returns
 * 
 * An instance of Opts containing the parsed command-line arguments
 */
pub fn parse_args() -> Opts {
    Opts::parse()
}