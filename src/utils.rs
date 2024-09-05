use std::path::Path;
use image::ImageFormat;

pub fn calculate_new_dimensions(width: u32, height: u32) -> (u32, u32) {
    let max_dimension = 1280; // Reduced from 1920 to 1280 for smaller file sizes
    if width > height {
        let new_width = width.min(max_dimension);
        let new_height = (height as f32 * (new_width as f32 / width as f32)) as u32;
        (new_width, new_height)
    } else {
        let new_height = height.min(max_dimension);
        let new_width = (width as f32 * (new_height as f32 / height as f32)) as u32;
        (new_width, new_height)
    }
}

pub fn determine_output_format(output_path: &Path) -> ImageFormat {
    match output_path.extension().and_then(|ext| ext.to_str()) {
        Some("png") => ImageFormat::Png,
        _ => ImageFormat::Jpeg,
    }
}