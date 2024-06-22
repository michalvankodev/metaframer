use anyhow::{Context, Result};
use clap::Parser;
use dirs::{self, config_dir};
use handlebars::Handlebars;
use log::{debug, error};
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

    #[command(flatten)]
    verbose: clap_verbosity_flag::Verbosity,
}

fn main() -> Result<()> {
    let args = CliArgs::parse();
    env_logger::Builder::new()
        .filter_level(args.verbose.log_level_filter())
        .init();

    let config_dir = config_dir().unwrap();
    let templates_path = config_dir.join("metaframer/templates");
    debug!("{:?} templates path", templates_path);

    let mut handlebars = Handlebars::new();
    // TODO use custom templates ... like custom from .config/file
    handlebars
        .register_template_file("default", templates_path.join("default.svg"))
        .with_context(|| {
            format!(
                "could not read template file`{:?}` in `{:?}`",
                "default.svg".to_string(),
                templates_path
            )
        })?;

    handlebars
        .register_template_file("Camera", templates_path.join("camera-icon.svg"))
        .with_context(|| {
            format!(
                "could not read template file`{:?}` in {:?}",
                "camera-icon.svg".to_string(),
                templates_path
            )
        })?;

    handlebars
        .register_template_file("Aperture", templates_path.join("aperture-icon.svg"))
        .with_context(|| {
            format!(
                "could not read template file`{:?}`",
                "aperture-icon.svg".to_string()
            )
        })?;
    handlebars
        .register_template_file(
            "ShutterSpeed",
            templates_path.join("shutter-speed-icon.svg"),
        )
        .with_context(|| {
            format!(
                "could not read template file`{:?}`",
                "shutter-speed-icon.svg".to_string()
            )
        })?;
    handlebars
        .register_template_file("FocalLength", templates_path.join("focal-length-icon.svg"))
        .with_context(|| {
            format!(
                "could not read template file`{:?}`",
                "focal-length-icon.svg".to_string()
            )
        })?;
    handlebars
        .register_template_file("Iso", templates_path.join("iso-icon.svg"))
        .with_context(|| {
            format!(
                "could not read template file`{:?}`",
                "iso-icon.svg".to_string()
            )
        })?;
    debug!("Files: {:?}", args.paths);
    debug!("Resolution: {:?}", args.resolution);

    let paths = args.paths.clone();

    for path in &paths {
        match process_file(&handlebars, &args, path) {
            Ok(..) => {}
            Err(error) => {
                error!("{:?}", error)
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
        debug!(
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
