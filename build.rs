use std::fs;
use std::path::Path;

fn main() {
    let config_dir = dirs::config_dir().unwrap();
    let dest_path = Path::new(&config_dir).join("metaframer/templates");

    // Create the destination directory if it doesn't exist
    fs::create_dir_all(&dest_path).unwrap();

    // Copy the template files to the destination directory
    for entry in fs::read_dir("templates").unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        if path.is_file() {
            let filename = path.file_name().unwrap();
            fs::copy(&path, dest_path.join(filename)).unwrap();
        }
    }

    // Tell Cargo to rerun the build script if the templates change
    println!("cargo:rerun-if-changed=templates");
}
