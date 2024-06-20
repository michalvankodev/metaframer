use anyhow::{Context, Result};
use clap::Parser;
use handlebars::Handlebars;
use resolution::Resolution;
use std::{fs::File, io::BufReader, path::PathBuf};

use crate::resolution::get_frame_width;

mod framer;
mod resolution;

#[derive(Parser)]
#[command(version, about, long_about = None)] // Read from `Cargo.toml`
struct CliArgs {
    paths: Vec<std::path::PathBuf>,

    /// Resolution to which the frame should be adjusted, width of the generated frame would be calculated according to this value
    #[arg(short, long, value_enum, default_value = "1080p")]
    resolution: Resolution,

    #[arg(short, long)]
    portrait: bool,

    #[arg(long = "height", default_value_t = 40)]
    frame_height: u8,

    #[arg(short, long)]
    inset: bool,
}

fn main() -> Result<()> {
    let args = CliArgs::parse();

    let mut handlebars = Handlebars::new();
    // TODO use custom templates ... like custom from .config/file
    handlebars
        .register_template_file("default", "./src/templates/default.svg")
        .with_context(|| {
            format!(
                "could not read template file`{:?}`",
                "default.svg".to_string()
            )
        })?;

    handlebars
        .register_template_file("Camera", "./src/templates/camera-icon.svg")
        .with_context(|| {
            format!(
                "could not read template file`{:?}`",
                "camera-icon.svg".to_string()
            )
        })?;

    handlebars
        .register_template_file("Aperture", "./src/templates/aperture-icon.svg")
        .with_context(|| {
            format!(
                "could not read template file`{:?}`",
                "aperture-icon.svg".to_string()
            )
        })?;
    handlebars
        .register_template_file("ShutterSpeed", "./src/templates/shutter-speed-icon.svg")
        .with_context(|| {
            format!(
                "could not read template file`{:?}`",
                "shutter-speed-icon.svg".to_string()
            )
        })?;
    handlebars
        .register_template_file("FocalLength", "./src/templates/focal-length-icon.svg")
        .with_context(|| {
            format!(
                "could not read template file`{:?}`",
                "focal-length-icon.svg".to_string()
            )
        })?;
    handlebars
        .register_template_file("Iso", "./src/templates/iso-icon.svg")
        .with_context(|| {
            format!(
                "could not read template file`{:?}`",
                "iso-icon.svg".to_string()
            )
        })?;
    println!("Files: {:?}", args.paths);
    println!("Resolution: {:?}", args.resolution);

    let paths = args.paths.clone();

    for path in &paths {
        match process_file(&handlebars, &args, path) {
            Ok(..) => {}
            Err(error) => {
                eprintln!("{:?}", error)
            }
        }
    }

    Ok(())
}

fn get_frame_path(path: &PathBuf) -> PathBuf {
    let mut frame_path = path.clone();
    let orig_file_stem = path.file_stem().unwrap();
    frame_path.set_file_name(format!("{}_frame", orig_file_stem.to_str().unwrap()));
    frame_path.set_extension("svg");

    frame_path
}

fn process_file(handlebars: &Handlebars<'_>, args: &CliArgs, path: &PathBuf) -> Result<()> {
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
    let excluded_height = if args.inset { 0 } else { args.frame_height };
    let frame_width = get_frame_width(args.resolution, args.portrait, dimensions, excluded_height);
    let frame_data = framer::get_frame_data((frame_width, args.frame_height as u32), &exif)?;

    let mut output_file = File::create(get_frame_path(&path))?;
    handlebars.render_to_write("default", &frame_data, &mut output_file)?;
    Ok(())
}
