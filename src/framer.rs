use anyhow::Result;
use exif::{Exif, In, Tag};
use serde::Serialize;

#[derive(Serialize)]
pub struct FrameData {
    pub width: u32,
    pub shutter_speed: String,
    pub aperture: String,
    pub focal_length: String,
    pub camera: String,
    // pub aperture: String;
}

// TODO Calculate width according to the aspect ratio of the image?
pub fn get_frame_data(_width: u32, exif: &Exif) -> Result<FrameData, anyhow::Error> {
    Ok(FrameData {
        width: 1000,
        shutter_speed: get_shutter_speed(&exif),
        aperture: get_aperture(&exif),
        focal_length: get_focal_length(&exif),
        camera: get_camera(&exif),
    })
}

fn get_shutter_speed(exif: &Exif) -> String {
    let field = exif.get_field(Tag::ExposureTime, In::PRIMARY);
    match field {
        Some(value) => format!("{}", value.display_value().with_unit(exif)),
        None => "N/A".to_string(),
    }
}

fn get_aperture(exif: &Exif) -> String {
    let field = exif.get_field(Tag::FNumber, In::PRIMARY);
    match field {
        Some(value) => format!("{}", value.display_value().with_unit(exif)),
        None => "N/A".to_string(),
    }
}

fn get_focal_length(exif: &Exif) -> String {
    let field = exif.get_field(Tag::FocalLength, In::PRIMARY);
    match field {
        Some(value) => format!("{}", value.display_value().with_unit(exif)),
        None => "N/A".to_string(),
    }
}

fn get_camera(exif: &Exif) -> String {
    let brand = exif.get_field(Tag::Make, In::PRIMARY);
    let model = exif.get_field(Tag::Model, In::PRIMARY);

    match (brand, model) {
        (Some(brand), Some(model)) => format!(
            "{} {}",
            brand.display_value().with_unit(exif),
            model.display_value().with_unit(exif)
        ),
        _ => "N/A".to_string(),
    }
}
