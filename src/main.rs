use anyhow::{Context, Result};
use clap::Parser;
use std::{fs::File, io::BufReader, path::PathBuf};

use crate::framer::Dimensions;

mod framer;

#[derive(Parser)]
struct CliArgs {
    path: std::path::PathBuf,
}

fn main() -> Result<()> {
    let args = CliArgs::parse();

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

    framer::generate_frame(
        get_frame_path(&path),
        Dimensions {
            width: 300,
            height: 300,
        },
    )?;

    Ok(())
}

fn get_frame_path(path: &PathBuf) -> PathBuf {
    let mut frame_path = path.clone();
    let orig_file_stem = path.file_stem().unwrap();
    frame_path.set_file_name(format!("{}_frame", orig_file_stem.to_str().unwrap()));
    frame_path.set_extension("png");

    frame_path
}
