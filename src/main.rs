use anyhow::{Context, Result};
use clap::{builder::PossibleValue, Parser, ValueEnum};
use handlebars::Handlebars;
use std::{fs::File, io::BufReader, path::PathBuf};

mod framer;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Debug)]
enum Resolution {
    HD,
    FullHD,
    QuadHD,
    UHD,
    FourK,
    EightK,
}

impl Resolution {
    fn dimensions(&self) -> (u32, u32) {
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

#[derive(Parser)]
#[command(version, about, long_about = None)] // Read from `Cargo.toml`
struct CliArgs {
    path: std::path::PathBuf,

    /// Resolution to which the frame should be adjusted, width of the generated frame would be calculated according to this value
    #[arg(short, long, value_enum, default_value = "1080p")]
    resolution: Resolution,

    #[arg(short, long)]
    portrait: bool,
    // TODO maybe we want the frame to be put inside of the image, then we don't want to exctract the genereated frame
}

fn main() -> Result<()> {
    let args = CliArgs::parse();

    let mut handlebars = Handlebars::new();
    // TODO use custom templates
    handlebars
        .register_template_file("default", "./src/templates/default.svg")
        .with_context(|| {
            format!(
                "could not read template file`{:?}`",
                "default.svg".to_string()
            )
        })?;

    println!("File: {:?}", args.path);
    println!("Resolution: {:?}", args.resolution);

    let path = args.path;
    let file = File::open(path.clone())
        .with_context(|| format!("could not read file `{:?}`", path.clone()))?;
    let mut bufreader = BufReader::new(&file);
    let exifreader = exif::Reader::new();
    let exif = exifreader
        .read_from_container(&mut bufreader)
        .with_context(|| format!("file `{:?}` is not a valid image", path.clone()))?;
    for f in exif.fields() {
        println!(
            "{} {} {}",
            f.tag,
            f.ifd_num,
            f.display_value().with_unit(&exif)
        );
    }

    let dimensions = image::image_dimensions(path.clone())?;
    let frame_width = get_frame_width(args.resolution, args.portrait, dimensions);
    let frame_data = framer::get_frame_data(frame_width, &exif)?;

    let mut output_file = File::create(get_frame_path(&path))?;
    handlebars.render_to_write("default", &frame_data, &mut output_file)?;

    Ok(())
}

fn get_frame_path(path: &PathBuf) -> PathBuf {
    let mut frame_path = path.clone();
    let orig_file_stem = path.file_stem().unwrap();
    frame_path.set_file_name(format!("{}_frame", orig_file_stem.to_str().unwrap()));
    frame_path.set_extension("svg");

    frame_path
}

fn get_frame_width(
    resolution: Resolution,
    is_portrait: bool,
    (o_width, o_height): (u32, u32),
) -> u32 {
    let o_is_landscape = o_width > o_height;
    let frame_dimensions = resolution.dimensions();

    // Rotate if we are generating for portrait frame
    let (width, height) = if is_portrait {
        (frame_dimensions.1, frame_dimensions.0)
    } else {
        frame_dimensions
    };

    match o_is_landscape {
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
    let frame_width = get_frame_width(Resolution::FullHD, false, portrait_dimensions);
    let expected_width = 760. / 1280. * 1080.;
    assert_eq!(frame_width, expected_width as u32)
}

#[test]
fn test_get_frame_width_with_portrait_for_portait() {
    let portrait_dimensions = (760, 1280);
    let frame_width = get_frame_width(Resolution::FullHD, true, portrait_dimensions);
    let expected_width = 1080;
    assert_eq!(frame_width, expected_width)
}

#[test]
fn test_get_frame_width_with_landscape_for_landscape() {
    let landscape_dimensions = (1280, 720);
    let frame_width = get_frame_width(Resolution::FullHD, false, landscape_dimensions);
    let expected_width = 1920;
    assert_eq!(frame_width, expected_width as u32)
}

#[test]
fn test_get_frame_width_with_landscape_for_portrait() {
    let landscape_dimensions = (1280, 720);
    let frame_width = get_frame_width(Resolution::FullHD, true, landscape_dimensions);
    let expected_width = 1280. / 720. * 1080.;
    assert_eq!(frame_width, expected_width as u32)
}
