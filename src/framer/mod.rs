pub mod positions;
pub mod text_values;

use anyhow::Result;
use exif::Exif;
use serde::Serialize;

use self::{
    positions::{get_positions, PositionedValue},
    text_values::{get_text_values, TextValuesKeys},
};

pub struct FrameSettings {
    letter_width: i32,
    border: i32,
    icon_size: i32,
}

#[derive(Serialize)]
pub struct FrameData {
    pub width: u32,
    pub height: u32,
    pub values: Vec<PositionedValue>,
}

pub fn get_frame_data(
    (width, height): (u32, u32),
    exif: &Exif,
) -> Result<FrameData, anyhow::Error> {
    let text_values = get_text_values(&exif);
    let left_display_order = vec![TextValuesKeys::Camera];
    let right_display_order = vec![
        TextValuesKeys::Aperture,
        TextValuesKeys::ShutterSpeed,
        TextValuesKeys::FocalLength,
        TextValuesKeys::Iso,
    ];
    let values = get_positions(
        &text_values,
        width,
        &left_display_order,
        &right_display_order,
    );

    Ok(FrameData {
        width,
        height,
        values,
    })
}

// TODO
// 2. How to scale / wrap information when width is not enough
// 3. How to display values in order rather than just fixed positions
