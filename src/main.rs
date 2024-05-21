use anyhow::{Context, Result};
use clap::Parser;
use handlebars::Handlebars;
use std::{fs::File, io::BufReader, path::PathBuf};

mod framer;

#[derive(Parser)]
struct CliArgs {
    path: std::path::PathBuf,
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
    let frame_data = framer::get_frame_data(dimensions.0, &exif)?;

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
