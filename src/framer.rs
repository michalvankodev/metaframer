use anyhow::Result;
use exif::{Exif, In, Tag};
use serde::Serialize;

pub struct FrameSettings {
    letter_width: i32,
    border: i32,
    icon_size: i32,
}

// enum PositionAlign {
//     Left,
//     Right,
// }

#[derive(Serialize, Debug, PartialEq)]
pub struct Positions {
    /// Vector of the `x` positions of icons in their order of display
    icons: Vec<i32>,

    /// Vector of the `x` positions of the labels/values in their order of display
    values: Vec<i32>,
}

#[derive(Serialize)]
pub struct TextValues {
    pub shutter_speed: String,
    pub aperture: String,
    pub focal_length: String,
    pub iso: String,
    pub camera: String,
    // pub lens: String,
}

#[derive(Copy, Clone)]
enum TextValuesKeys {
    Camera,
    Aperture,
    ShutterSpeed,
    FocalLength,
    Iso,
}

impl TextValues {
    // Function to get a property by its key
    fn get_property(&self, key: &TextValuesKeys) -> &String {
        match key {
            TextValuesKeys::Camera => &self.camera,
            TextValuesKeys::ShutterSpeed => &self.shutter_speed,
            TextValuesKeys::Aperture => &self.aperture,
            TextValuesKeys::FocalLength => &self.focal_length,
            TextValuesKeys::Iso => &self.iso,
        }
    }
}

#[derive(Serialize)]
pub struct FrameData {
    pub width: u32,
    pub height: u32,
    pub left_aligned_positions: Positions,
    pub right_aligned_positions: Positions,
    pub text_values: TextValues,
}

pub fn get_frame_data(
    (width, height): (u32, u32),
    exif: &Exif,
) -> Result<FrameData, anyhow::Error> {
    let text_values = TextValues {
        shutter_speed: get_shutter_speed(&exif),
        aperture: get_aperture(&exif),
        focal_length: get_focal_length(&exif),
        camera: get_camera(&exif),
        iso: get_iso(&exif),
    };

    let (left_aligned_positions, right_aligned_positions) = get_positions(&text_values, width);

    Ok(FrameData {
        width,
        height,
        text_values,
        left_aligned_positions,
        right_aligned_positions,
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

fn get_positions(text_values: &TextValues, width: u32) -> (Positions, Positions) {
    let left_display_order = vec![TextValuesKeys::Camera];
    let right_display_order = vec![
        TextValuesKeys::Aperture,
        TextValuesKeys::ShutterSpeed,
        TextValuesKeys::FocalLength,
        TextValuesKeys::Iso,
    ];
    let frame_settings = FrameSettings {
        border: 10,
        icon_size: 30,
        letter_width: 10,
    };

    let left_positions =
        get_left_aligned_positions(&frame_settings, &text_values, &left_display_order);

    let right_positions =
        get_right_aligned_positions(&frame_settings, &text_values, &right_display_order, width);

    (left_positions, right_positions)
}

fn get_left_aligned_positions(
    FrameSettings {
        letter_width,
        border,
        icon_size,
    }: &FrameSettings,
    text_values: &TextValues,
    display_order: &Vec<TextValuesKeys>,
) -> Positions {
    let mut icons: Vec<i32> = vec![];
    let mut values: Vec<i32> = vec![];
    let mut last_prop: Option<&TextValuesKeys> = None;

    for prop in display_order {
        let last_value_position = values.last().unwrap_or(&0);
        let last_value_size = if let Some(last_prop_value) = last_prop {
            text_values.get_property(&last_prop_value).len()
        } else {
            0
        };
        let next_icon_position =
            last_value_position + last_value_size as i32 * letter_width + border;
        icons.push(next_icon_position);
        values.push(next_icon_position + icon_size + border);
        last_prop = Some(prop);
    }

    return Positions { icons, values };
}

fn get_right_aligned_positions(
    FrameSettings {
        letter_width,
        border,
        icon_size,
    }: &FrameSettings,
    text_values: &TextValues,
    display_order: &Vec<TextValuesKeys>,
    width: u32,
) -> Positions {
    let mut icons: Vec<i32> = vec![];
    let mut values: Vec<i32> = vec![];
    let mut reversed_order = display_order.clone();
    reversed_order.reverse();

    for prop in reversed_order {
        let last_icon_position = icons.last().unwrap_or(&0);
        let text = text_values.get_property(&prop).len();
        let value = last_icon_position + border + text as i32 * letter_width;
        let icon = value + border + icon_size;
        values.push(value);
        icons.push(icon);
    }

    let icons = icons
        .iter()
        .map(|offset| width as i32 - offset)
        .rev()
        .collect();
    let values = values
        .iter()
        .map(|offset| width as i32 - offset)
        .rev()
        .collect();

    return Positions { icons, values };
}

#[test]
fn test_get_left_aligned_positions() {
    let frame_settings = FrameSettings {
        border: 10,
        icon_size: 30,
        letter_width: 10,
    };
    let FrameSettings {
        letter_width,
        border,
        icon_size,
    } = frame_settings;

    let text_values = TextValues {
        camera: "My camera 1234".to_string(),
        aperture: "f/8".to_string(),
        shutter_speed: "1/250s".to_string(),
        focal_length: "18.1mm".to_string(),
        iso: "3600".to_string(),
    };

    let display_order = vec![
        TextValuesKeys::Camera,
        TextValuesKeys::Aperture,
        TextValuesKeys::ShutterSpeed,
        TextValuesKeys::FocalLength,
        TextValuesKeys::Iso,
    ];

    // 14 * 10 + borders
    let first_value = border + icon_size + border;
    let second_icon = first_value + 14 * letter_width + border;
    let second_value = second_icon + icon_size + border;
    let third_icon = second_value + 3 * letter_width + border;
    let third_value = third_icon + icon_size + border;
    let fourth_icon = third_value + 6 * letter_width + border;
    let fourth_value = fourth_icon + icon_size + border;
    let fifth_icon = fourth_value + 6 * letter_width + border;
    let fifth_value = fifth_icon + icon_size + border;

    let expected_positions = Positions {
        icons: vec![10, second_icon, third_icon, fourth_icon, fifth_icon],
        values: vec![
            first_value,
            second_value,
            third_value,
            fourth_value,
            fifth_value,
        ],
    };

    assert_eq!(
        get_left_aligned_positions(&frame_settings, &text_values, &display_order),
        expected_positions
    );
}

#[test]
fn test_get_right_aligned_positions() {
    let frame_settings = FrameSettings {
        border: 10,
        icon_size: 30,
        letter_width: 10,
    };
    let FrameSettings {
        letter_width,
        border,
        icon_size,
    } = frame_settings;

    let text_values = TextValues {
        camera: "My camera 1234".to_string(),
        aperture: "f/8".to_string(),
        shutter_speed: "1/250s".to_string(),
        focal_length: "18.1mm".to_string(),
        iso: "3600".to_string(),
    };

    let display_order = vec![
        TextValuesKeys::Aperture,
        TextValuesKeys::ShutterSpeed,
        TextValuesKeys::FocalLength,
        TextValuesKeys::Iso,
    ];
    let width = 1000;

    let first_value = border + letter_width * 4;
    let first_icon = first_value + border + icon_size;
    let second_value = first_icon + border + letter_width * 6;
    let second_icon = second_value + border + icon_size;
    let third_value = second_icon + border + letter_width * 6;
    let third_icon = third_value + border + icon_size;
    let fourth_value = third_icon + border + letter_width * 3;
    let fourth_icon = fourth_value + border + icon_size;

    let mut icons: Vec<i32> = vec![first_icon, second_icon, third_icon, fourth_icon]
        .iter()
        .map(|offset| width - offset)
        .collect();
    icons.reverse();
    let mut values: Vec<i32> = vec![first_value, second_value, third_value, fourth_value]
        .iter()
        .map(|offset| width - offset)
        .collect();
    values.reverse();
    let expected_positions = Positions { icons, values };

    assert_eq!(
        get_right_aligned_positions(&frame_settings, &text_values, &display_order, width as u32),
        expected_positions
    );
}

// TODO
// 2. How to scale / wrap information when width is not enough
// 3. How to display values in order rather than just fixed positions
