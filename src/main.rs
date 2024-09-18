mod cli;
mod compressor;
mod utils;

use std::path::Path;
use log::{info, error};
use env_logger::Env;

/**
 * The main function of the image compressor CLI tool.
 * 
 * This function does the following:
 * 1. Parses command-line arguments
 * 2. Checks if the input directory exists
 * 3. Creates the output directory if it doesn't exist
 * 4. Initiates the image compression process
 * 5. Prints a completion message
 */
fn main() -> Result<(), Box<dyn std::error::Error>> {
     env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    let opts = cli::parse_args();
    
    let input_dir = Path::new(&opts.input);
    let output_dir = Path::new(&opts.output);

    if !input_dir.exists() {
         error!("Input directory does not exist: {}", input_dir.display());
        return Err("Input directory does not exist".into());
    }

    std::fs::create_dir_all(output_dir)?;

    compressor::process_directory(input_dir, output_dir, opts.quality, opts.max_dimension)?;

    info!("Compression complete. Compressed images are in: {}", output_dir.display());
    Ok(())
}