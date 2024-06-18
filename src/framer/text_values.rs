use exif::{Exif, In, Tag};
use serde::Serialize;

#[derive(Serialize)]
pub struct TextValues {
    pub shutter_speed: String,
    pub aperture: String,
    pub focal_length: String,
    pub iso: String,
    pub camera: String,
    // pub lens: String,
}

#[derive(Copy, Clone, Serialize, Debug, PartialEq, Eq)]
pub enum TextValuesKeys {
    Camera,
    Aperture,
    ShutterSpeed,
    FocalLength,
    Iso,
}

impl TextValues {
    // Function to get a property by its key
    pub fn get_property(&self, key: &TextValuesKeys) -> &String {
        match key {
            TextValuesKeys::Camera => &self.camera,
            TextValuesKeys::ShutterSpeed => &self.shutter_speed,
            TextValuesKeys::Aperture => &self.aperture,
            TextValuesKeys::FocalLength => &self.focal_length,
            TextValuesKeys::Iso => &self.iso,
        }
    }
}

pub fn get_text_values(exif: &Exif) -> TextValues {
    TextValues {
        shutter_speed: get_shutter_speed(&exif),
        aperture: get_aperture(&exif),
        focal_length: get_focal_length(&exif),
        camera: get_camera(&exif),
        iso: get_iso(&exif),
    }
}

pub fn get_shutter_speed(exif: &Exif) -> String {
    let field = exif.get_field(Tag::ExposureTime, In::PRIMARY);
    match field {
        Some(value) => format!("{}", value.display_value().with_unit(exif)),
        None => "N/A".to_string(),
    }
}

pub fn get_aperture(exif: &Exif) -> String {
    let field = exif.get_field(Tag::FNumber, In::PRIMARY);
    match field {
        Some(value) => format!("{}", value.display_value().with_unit(exif)),
        None => "N/A".to_string(),
    }
}

pub fn get_focal_length(exif: &Exif) -> String {
    let field = exif.get_field(Tag::FocalLength, In::PRIMARY);
    match field {
        Some(value) => format!("{}", value.display_value().with_unit(exif)),
        None => "N/A".to_string(),
    }
}

pub fn get_iso(exif: &Exif) -> String {
    let field = exif.get_field(Tag::PhotographicSensitivity, In::PRIMARY);
    match field {
        Some(value) => format!("{}", value.display_value().with_unit(exif)),
        None => "N/A".to_string(),
    }
}

pub fn get_camera(exif: &Exif) -> String {
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
