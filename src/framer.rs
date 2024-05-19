use std::path::PathBuf;

use anyhow::Result;
use image::{ImageBuffer, Rgb, RgbImage};

pub struct Dimensions {
    pub width: u32,
    pub height: u32,
}

pub fn generate_frame(
    path: PathBuf,
    dimensions: Dimensions,
) -> Result<ImageBuffer<Rgb<u8>, Vec<u8>>> {
    let image = RgbImage::new(dimensions.width, dimensions.height);

    image.save(path).unwrap();

    Ok(image)
}
