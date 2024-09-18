use std::path::Path;
use image::{GenericImageView, ImageFormat, imageops::FilterType};
use walkdir::WalkDir;
use indicatif::{ProgressBar, ProgressStyle};
use oxipng::{InFile, OutFile, Options};
use log::{info, warn};
use std::sync::Arc;
use rayon::prelude::*;

use crate::utils;

/**
 * Process all images in the given directory and its subdirectories
 * 
 * # Arguments
 * 
 * * `input_dir` - The path to the input directory
 * * `output_dir` - The path to the output directory
 * * `quality` - The quality setting for JPEG compression (0-100)
 * * `max_dimension` - The maximum dimension for the image
 * 
 * # Returns
 * 
 * A Result indicating success or containing an error
 */
pub fn process_directory(input_dir: &Path, output_dir: &Path, quality: u8, max_dimension: u32) -> Result<(), Box<dyn std::error::Error>> {
   let entries: Vec<_> = WalkDir::new(input_dir)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
        .collect();

    let progress_bar = Arc::new(ProgressBar::new(entries.len() as u64));
    progress_bar.set_style(ProgressStyle::default_bar()
        .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({eta})")
        .unwrap()
        .progress_chars("#>-"));

    entries.par_iter().for_each(|entry| {
        let path = entry.path();
        if let Some(extension) = path.extension() {
            if utils::is_supported_format(extension) {
                let relative_path = path.strip_prefix(input_dir).unwrap();
                let output_path = output_dir.join(relative_path);
                
                if let Some(parent) = output_path.parent() {
                    let _ = std::fs::create_dir_all(parent);
                }
                
                match compress_image(path, &output_path, quality, max_dimension) {
                    Ok(_) => info!("Compressed: {}", path.display()),
                    Err(e) => warn!("Failed to compress {}: {}", path.display(), e),
                }
            }
        }
        progress_bar.inc(1);
    });

    progress_bar.finish_with_message("Compression complete");
    Ok(())

}

/**
 * Compress a single image
 * 
 * # Arguments
 * 
 * * `input_path` - The path to the input image
 * * `output_path` - The path where the compressed image will be saved
 * * `quality` - The quality setting for JPEG compression (0-100)
 * * `max_dimension` - The maximum dimension for the image
 * 
 * # Returns
 * 
 * A Result indicating success or containing an ImageError
 */
fn compress_image(input_path: &Path, output_path: &Path, quality: u8, max_dimension: u32) -> Result<(), Box<dyn std::error::Error>> {
    let img = image::open(input_path)?;


    
    let (width, height) = img.dimensions();
    let (new_width, new_height) = utils::calculate_new_dimensions(width, height, max_dimension);

    let resized = img.resize(new_width, new_height, FilterType::Lanczos3);
    
    let output_format = utils::determine_output_format(output_path);

    match output_format {
        ImageFormat::Jpeg => {
            let mut output_file = std::fs::File::create(output_path)?;
            resized.write_to(&mut output_file, image::ImageOutputFormat::Jpeg(quality))?;
        },
         ImageFormat::Png => {
            resized.save(output_path)?;

            let input = InFile::Path(output_path.to_path_buf());
            let output = OutFile::Path(Some(output_path.to_path_buf()));
            
            oxipng::optimize(&input, &output, &Options::default())?;

        },
        _ => {
            resized.save_with_format(output_path, output_format)?;
        },
    }
    
    Ok(())
}

