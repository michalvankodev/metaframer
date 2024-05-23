use anyhow::Result;
use exif::{Exif, In, Tag};
use serde::Serialize;

#[derive(Serialize)]
pub struct FrameData {
    pub width: u32,
    pub shutter_speed: String,
    pub aperture: String,
    pub focal_length: String,
    pub iso: String,
    pub camera: String,
    // pub lens: String,
}

// TODO Calculate width according to the aspect ratio of the image?
pub fn get_frame_data(_width: u32, exif: &Exif) -> Result<FrameData, anyhow::Error> {
    Ok(FrameData {
        width: 1000,
        shutter_speed: get_shutter_speed(&exif),
        aperture: get_aperture(&exif),
        focal_length: get_focal_length(&exif),
        camera: get_camera(&exif),
        iso: get_iso(&exif),
        // lens: get_lens(&exif),
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

fn get_iso(exif: &Exif) -> String {
    let field = exif.get_field(Tag::PhotographicSensitivity, In::PRIMARY);
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
            brand
                .display_value()
                .with_unit(exif)
                .to_string()
                .trim_matches('"'),
            model
                .display_value()
                .with_unit(exif)
                .to_string()
                .trim_matches('"')
        ),
        _ => "N/A".to_string(),
    }
}

// TODO
// 1. Width of the generated frame
// 2. How to scale / wrap information when width is not enough
// 3. How to display values in order rather than just fixed positions
