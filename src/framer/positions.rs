use serde::Serialize;

use super::{
    text_values::{TextValues, TextValuesKeys},
    FrameSettings,
};

#[derive(Serialize, Debug, PartialEq, Clone)]
pub struct PositionedValue {
    text_position: i32,
    icon_position: i32,
    text: String,
    value_key: TextValuesKeys,
}

pub fn get_positions(
    text_values: &TextValues,
    width: u32,
    left_display_order: &Vec<TextValuesKeys>,
    right_display_order: &Vec<TextValuesKeys>,
) -> Vec<PositionedValue> {
    let frame_settings = FrameSettings {
        border: 10,
        icon_size: 30,
        letter_width: 10,
    };

    let left_positions =
        get_left_aligned_positions(&frame_settings, &text_values, left_display_order);

    let right_positions =
        get_right_aligned_positions(&frame_settings, &text_values, right_display_order, width);

    let positioned_values = [left_positions, right_positions].concat();
    return positioned_values;
}

fn get_left_aligned_positions<'a>(
    FrameSettings {
        letter_width,
        border,
        icon_size,
    }: &FrameSettings,
    text_values: &TextValues,
    display_order: &Vec<TextValuesKeys>,
) -> Vec<PositionedValue> {
    let mut positioned_values: Vec<PositionedValue> = vec![];

    for prop in display_order {
        let (icon_position, text_position) = match positioned_values.last() {
            Some(last_value) => {
                let last_text_len = last_value.text.len() as i32;
                let next_icon_position =
                    last_value.text_position + last_text_len * letter_width + border;
                let next_text_position = next_icon_position + icon_size + border;
                (next_icon_position, next_text_position)
            }
            None => (border.clone(), border + icon_size + border),
        };

        positioned_values.push(PositionedValue {
            text_position,
            icon_position,
            text: text_values.get_property(&prop).clone(),
            value_key: prop.clone(),
        })
    }

    return positioned_values;
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
) -> Vec<PositionedValue> {
    let mut positioned_values: Vec<PositionedValue> = vec![];
    let reversed_order = display_order.iter().rev();

    for prop in reversed_order {
        let last_icon_position = positioned_values
            .last()
            .map_or(&0, |value| &value.icon_position);

        let text = text_values.get_property(&prop).clone();
        let text_size = text.len();
        let text_position = last_icon_position + border + text_size as i32 * letter_width;
        let icon_position = text_position + border + icon_size;

        positioned_values.push(PositionedValue {
            text_position,
            icon_position,
            text,
            value_key: prop.clone(),
        })
    }

    positioned_values.iter_mut().for_each(|value| {
        value.text_position = width as i32 - value.text_position;
        value.icon_position = width as i32 - value.icon_position
    });
    positioned_values.reverse();

    return positioned_values;
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

    let expected_positions = vec![
        PositionedValue {
            text_position: first_value,
            icon_position: 10,
            text: text_values.camera.clone(),
            value_key: TextValuesKeys::Camera,
        },
        PositionedValue {
            text_position: second_value,
            icon_position: second_icon,
            text: text_values.aperture.clone(),
            value_key: TextValuesKeys::Aperture,
        },
        PositionedValue {
            text_position: third_value,
            icon_position: third_icon,
            text: text_values.shutter_speed.clone(),
            value_key: TextValuesKeys::ShutterSpeed,
        },
        PositionedValue {
            text_position: fourth_value,
            icon_position: fourth_icon,
            text: text_values.focal_length.clone(),
            value_key: TextValuesKeys::FocalLength,
        },
        PositionedValue {
            text_position: fifth_value,
            icon_position: fifth_icon,
            text: text_values.iso.clone(),
            value_key: TextValuesKeys::Iso,
        },
    ];

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

    let expected_positions = vec![
        PositionedValue {
            text_position: values[0],
            icon_position: icons[0],
            text: text_values.aperture.clone(),
            value_key: TextValuesKeys::Aperture,
        },
        PositionedValue {
            text_position: values[1],
            icon_position: icons[1],
            text: text_values.shutter_speed.clone(),
            value_key: TextValuesKeys::ShutterSpeed,
        },
        PositionedValue {
            text_position: values[2],
            icon_position: icons[2],
            text: text_values.focal_length.clone(),
            value_key: TextValuesKeys::FocalLength,
        },
        PositionedValue {
            text_position: values[3],
            icon_position: icons[3],
            text: text_values.iso.clone(),
            value_key: TextValuesKeys::Iso,
        },
    ];

    assert_eq!(
        get_right_aligned_positions(&frame_settings, &text_values, &display_order, width as u32),
        expected_positions
    );
}
