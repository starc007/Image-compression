use std::path::Path;
use image::ImageFormat;
use std::ffi::OsStr;

pub fn calculate_new_dimensions(width: u32, height: u32) -> (u32, u32) {
    let max_dimension = 1280;
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
        Some("jpg") | Some("jpeg") => ImageFormat::Jpeg,
        Some("png") => ImageFormat::Png,
        Some("gif") => ImageFormat::Gif,
        Some("webp") => ImageFormat::WebP,
        Some("tiff") => ImageFormat::Tiff,
        Some("tga") => ImageFormat::Tga,
        Some("bmp") => ImageFormat::Bmp,
        Some("ico") => ImageFormat::Ico,
        Some("hdr") => ImageFormat::Hdr,
        _ => ImageFormat::Jpeg, // Default to JPEG if unknown
    }
}

pub fn is_supported_format(extension: &OsStr) -> bool {
    let supported_formats = [
        "jpg", "jpeg", "png", "gif", "webp", "tiff", "tga", "bmp", "ico", "hdr",
    ];
    extension
        .to_str()
        .map(|ext| supported_formats.contains(&ext.to_lowercase().as_str()))
        .unwrap_or(false)
}