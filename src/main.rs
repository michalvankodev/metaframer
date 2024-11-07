use anyhow::{Context, Result};
use clap::Parser;
use handlebars::Handlebars;
use log::{debug, error};
use resolution::Resolution;
use std::{
    fs::File,
    io::BufReader,
    path::{Path, PathBuf},
};
use templates::{copy_default_template, init_templates_if_needed, register_templates};

use crate::resolution::get_frame_width;

mod framer;
mod resolution;
mod templates;

#[derive(Parser)]
#[command(version, about, long_about = None)] // Read from `Cargo.toml`
struct CliArgs {
    paths: Vec<std::path::PathBuf>,

    /// Resolution to which the frame should be adjusted, width of the generated frame would be calculated according to this value
    #[arg(short, long, value_enum, default_value = "1080p")]
    resolution: Resolution,

    /// Instructs metaframer to generate frames to be used in targetted portraits
    #[arg(short, long)]
    portrait: bool,

    /// Height that should be taken by the frame
    #[arg(long = "height", default_value_t = 40)]
    frame_height: u8,

    /// Instruct to overwrite default configuration and templates
    #[arg(long = "reset")]
    reset: bool,

    /// Instruct to overwrite default configuration and templates
    #[arg(short, long = "template", default_value = "default")]
    template_name: String,

    /// Specifies whether generated frames will not take any height of the target
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
    init_templates_if_needed()?;

    if args.reset {
        copy_default_template()?;
    }

    let mut handlebars = Handlebars::new();
    register_templates(&args.template_name, &mut handlebars)?;

    debug!("Files: {:?}", args.paths);
    debug!("Resolution: {:?}", args.resolution);
    debug!("Template name: {:?}", args.template_name);

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

fn get_frame_path(path: &Path) -> PathBuf {
    let mut frame_path = path.to_path_buf();
    let orig_file_stem = path.file_stem().unwrap();
    frame_path.set_file_name(format!("{}_frame", orig_file_stem.to_str().unwrap()));
    frame_path.set_extension("svg");

    frame_path
}

fn process_file(handlebars: &Handlebars<'_>, args: &CliArgs, path: &Path) -> Result<()> {
    let file = File::open(path).with_context(|| format!("could not read file `{:?}`", path))?;
    let mut bufreader = BufReader::new(&file);
    let exifreader = exif::Reader::new();
    let exif = exifreader
        .read_from_container(&mut bufreader)
        .with_context(|| format!("file `{:?}` is not a valid image", path))?;
    for f in exif.fields() {
        debug!(
            "{} {} {}",
            f.tag,
            f.ifd_num,
            f.display_value().with_unit(&exif)
        );
    }

    let dimensions = image::image_dimensions(path)?;
    let excluded_height = if args.inset { 0 } else { args.frame_height };
    let frame_width = get_frame_width(args.resolution, args.portrait, dimensions, excluded_height);
    let frame_data = framer::get_frame_data((frame_width, args.frame_height as u32), &exif)?;

    let mut output_file = File::create(get_frame_path(path))?;
    handlebars.render_to_write("main", &frame_data, &mut output_file)?;
    Ok(())
}
