use anyhow::{Context, Result};
use clap::Parser;
use std::{fs::File, io::BufReader};

#[derive(Parser)]
struct CliArgs {
    path: std::path::PathBuf,
}

fn main() -> Result<()> {
    // let pattern = std::env::args().nth(1).expect("no pattern given");
    // let path = std::env::args().nth(2).expect("no path given");

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

    Ok(())
}
