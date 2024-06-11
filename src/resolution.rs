use clap::{builder::PossibleValue, ValueEnum};

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Debug)]
pub enum Resolution {
    HD,
    FullHD,
    QuadHD,
    UHD,
    FourK,
    EightK,
}

impl Resolution {
    pub fn dimensions(&self) -> (u32, u32) {
        match *self {
            Resolution::HD => (1280, 720),
            Resolution::FullHD => (1920, 1080),
            Resolution::QuadHD => (2560, 1440),
            Resolution::UHD => (3840, 2160),
            Resolution::FourK => (4096, 2160),
            Resolution::EightK => (7680, 4320),
        }
    }
}

impl ValueEnum for Resolution {
    fn value_variants<'a>() -> &'a [Self] {
        &[
            Resolution::HD,
            Resolution::FullHD,
            Resolution::QuadHD,
            Resolution::UHD,
            Resolution::FourK,
            Resolution::EightK,
        ]
    }

    fn to_possible_value<'a>(&self) -> Option<PossibleValue> {
        Some(match self {
            Resolution::HD => PossibleValue::new("720p").help("HD resolution 1280x720"),
            Resolution::FullHD => PossibleValue::new("1080p").help("Full HD resolution 1920x1080"),
            Resolution::QuadHD => PossibleValue::new("2k").help("Quad HD resolution 2560x1440"),
            Resolution::UHD => PossibleValue::new("UHD").help("Ultra HD resolution 3840x2160"),
            Resolution::FourK => PossibleValue::new("4k").help("4k resolution 4096x2160"),
            Resolution::EightK => PossibleValue::new("8k").help("8k resolution 7680x4320"),
        })
    }
}

pub fn get_frame_width(
    resolution: Resolution,
    is_portrait: bool,
    (o_width, o_height): (u32, u32),
    excluded_height: u8,
) -> u32 {
    let frame_dimensions = resolution.dimensions();

    // Rotate if we are generating for portrait frame
    let (width, height) = if is_portrait {
        (
            frame_dimensions.1,
            frame_dimensions.0 - u32::from(excluded_height),
        )
    } else {
        (
            frame_dimensions.0,
            frame_dimensions.1 - u32::from(excluded_height),
        )
    };

    let has_bigger_aspect_ratio = o_width / o_height > width / height;

    match has_bigger_aspect_ratio {
        false => {
            let scale = f64::from(height) / f64::from(o_height);
            return (scale * f64::from(o_width)) as u32;
        }
        true => width,
    }
}

#[test]
fn test_get_frame_width_with_portrait_for_landscape() {
    let portrait_dimensions = (760, 1280);
    let frame_width = get_frame_width(Resolution::FullHD, false, portrait_dimensions, 40);
    let x = (1080. - 40.) / 1280.0;
    let expected_width = 760. * x;
    println!("expected width: {}", expected_width);
    assert_eq!(frame_width, expected_width as u32)
}

#[test]
fn test_get_frame_width_with_portrait_for_portait() {
    let portrait_dimensions = (760, 1280);
    let frame_width = get_frame_width(Resolution::FullHD, true, portrait_dimensions, 40);
    let x = (1920. - 40.) / 1280.;
    let expected_width = 760. * x;
    println!("expected width: {}", expected_width);
    assert_eq!(frame_width, expected_width as u32)
}

#[test]
fn test_get_frame_width_with_portrait_for_portrait_with_bigger_aspect_ratio() {
    let portrait_dimensions = (720, 1280);
    let frame_width = get_frame_width(Resolution::FullHD, true, portrait_dimensions, 40);
    let x = (1920. - 40.) / 1280.;
    let expected_width = 720. * x;
    assert_eq!(frame_width, expected_width as u32)
}

#[test]
fn test_get_frame_width_with_landscape_for_landscape_with_bigger_aspect_ratio() {
    let landscape_dimensions = (1280, 720);
    let frame_width = get_frame_width(Resolution::FullHD, false, landscape_dimensions, 40);
    let x = (1080. - 40.) / 720.;
    let expected_width = 1280. * x;
    assert_eq!(frame_width, expected_width as u32)
}

#[test]
fn test_get_frame_width_with_landscape_for_portrait() {
    let landscape_dimensions = (1280, 720);
    let frame_width = get_frame_width(Resolution::FullHD, true, landscape_dimensions, 40);
    let expected_width = 1080.;
    assert_eq!(frame_width, expected_width as u32)
}

#[test]
fn test_get_frame_width_with_landscape_for_portrait_with_bigger_img() {
    let landscape_dimensions = (2000, 720);
    let frame_width = get_frame_width(Resolution::FullHD, true, landscape_dimensions, 40);
    let expected_width = 1080.;
    assert_eq!(frame_width, expected_width as u32)
}

#[test]
fn test_get_frame_width_with_portrait_for_landscape_with_bigger_img() {
    let portrait_dimensions = (2080, 3800);
    let frame_width = get_frame_width(Resolution::FullHD, false, portrait_dimensions, 40);
    let x = (1080. - 40.) / 3800.0;
    let expected_width = 2080. * x;
    println!("expected width: {}", expected_width);
    assert_eq!(frame_width, expected_width as u32)
}
