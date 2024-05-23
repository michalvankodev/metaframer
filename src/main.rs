use anyhow::{Context, Result};
use clap::{builder::PossibleValue, Parser, ValueEnum};
use handlebars::Handlebars;
use std::{fs::File, io::BufReader, path::PathBuf};

mod framer;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Debug)]
enum Resolution {
    Hd,
    FullHd,
    QuadHd,
    UHd,
}

impl ValueEnum for Resolution {
    fn value_variants<'a>() -> &'a [Self] {
        &[
            Resolution::Hd,
            Resolution::FullHd,
            Resolution::QuadHd,
            Resolution::UHd,
        ]
    }

    fn to_possible_value<'a>(&self) -> Option<PossibleValue> {
        Some(match self {
            Resolution::Hd => PossibleValue::new("720p").help("HD resolution 1280x720"),
            Resolution::FullHd => PossibleValue::new("1080p").help("Full HD resolution 1920x1080"),
            Resolution::QuadHd => PossibleValue::new("2k").help("Quad HD resolution 2560x1440"),
            Resolution::UHd => PossibleValue::new("4k").help("Ultra HD resolution 3840x2160"),
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
    let frame_width = get_frame_width(args.resolution, dimensions);
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

fn get_frame_width(_resolution: Resolution, (o_width, o_height): (u32, u32)) -> u32 {
    let is_landscape = o_width > o_height;

    match is_landscape {
        false => {
            let scale = f64::from(1080) / f64::from(o_height);
            return (scale * f64::from(o_width)) as u32;
        }
        true => 1920,
    }
}

#[test]
fn test_get_frame_width_for_portrait() {
    let portrait_dimensions = (760, 1280);
    let frame_width = get_frame_width(Resolution::FullHd, portrait_dimensions);
    let expected_width = 760. / 1280. * 1080.;
    assert_eq!(frame_width, expected_width as u32)
}

#[test]
fn test_get_frame_width_for_landscape() {
    let landscape_dimensions = (1280, 720);
    let frame_width = get_frame_width(Resolution::FullHd, landscape_dimensions);
    let expected_width = 1920;
    assert_eq!(frame_width, expected_width as u32)
}
