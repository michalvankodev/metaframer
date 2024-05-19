use clap::Parser;
use std::{
    fs::File,
    io::{BufReader, Error},
};

#[derive(Parser)]
struct CliArgs {
    path: std::path::PathBuf,
}

fn main() {
    // let pattern = std::env::args().nth(1).expect("no pattern given");
    // let path = std::env::args().nth(2).expect("no path given");

    let args = CliArgs::parse();

    println!("File: {:?}", args.path);

    let file = File::open(args.path).unwrap();
    let mut bufreader = BufReader::new(&file);
    let exifreader = exif::Reader::new();
    let exif = exifreader.read_from_container(&mut bufreader).unwrap();
    for f in exif.fields() {
        println!(
            "{} {} {}",
            f.tag,
            f.ifd_num,
            f.display_value().with_unit(&exif)
        );
    }

    ()
}
