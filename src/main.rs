mod cli;
mod compressor;
mod utils;

use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let opts = cli::parse_args();
    
    let input_dir = Path::new(&opts.input);
    let output_dir = Path::new(&opts.output);

    if !input_dir.exists() {
        return Err("Input directory does not exist".into());
    }

    std::fs::create_dir_all(output_dir)?;

    compressor::process_directory(input_dir, output_dir, opts.quality)?;

    println!("Compression complete. Compressed images are in: {}", output_dir.display());
    Ok(())
}