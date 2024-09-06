use std::path::Path;
use image::{GenericImageView, ImageFormat, imageops::FilterType};
use walkdir::WalkDir;

use crate::utils;

pub fn process_directory(input_dir: &Path, output_dir: &Path, quality: u8) -> Result<(), Box<dyn std::error::Error>> {
    for entry in WalkDir::new(input_dir).into_iter().filter_map(|e| e.ok()) {
        let path = entry.path();
        if path.is_file() {
            if let Some(extension) = path.extension() {
                if utils::is_supported_format(extension) {
                    let relative_path = path.strip_prefix(input_dir)?;
                    let output_path = output_dir.join(relative_path);
                    
                    if let Some(parent) = output_path.parent() {
                        std::fs::create_dir_all(parent)?;
                    }
                    
                    compress_image(path, &output_path, quality)?;
                    println!("Compressed: {}", path.display());
                }
            }
        }
    }
    Ok(())
}

fn compress_image(input_path: &Path, output_path: &Path, quality: u8) -> Result<(), image::ImageError> {
    let img = image::open(input_path)?;
    
    let (width, height) = img.dimensions();
    let (new_width, new_height) = utils::calculate_new_dimensions(width, height);

    let resized = img.resize(new_width, new_height, FilterType::Lanczos3);
    
    let output_format = utils::determine_output_format(output_path);

    match output_format {
        ImageFormat::Jpeg => {
            resized.save_with_format(output_path, output_format)?;
            let mut output_file = std::fs::File::create(output_path)?;
            resized.write_to(&mut output_file, image::ImageOutputFormat::Jpeg(quality))?;
        },
        ImageFormat::Png => {
            let mut output_file = std::fs::File::create(output_path)?;
            resized.write_to(&mut output_file, image::ImageOutputFormat::Png)?;
        },
        _ => {
            resized.save_with_format(output_path, output_format)?;
        },
    }
    
    Ok(())
}