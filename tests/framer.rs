use std::{fs, path::Path};

use anyhow::Result;
use assert_cmd::Command;

#[test]
fn generates_frame() -> Result<()> {
    let mut cmd = Command::cargo_bin("metaframer")?;

    cmd.arg("tests/assets/image.jpg");
    assert!(Path::new("tests/assets/image_frame.svg").exists());

    // Clean up
    fs::remove_file("tests/assets/image_frame.svg").unwrap();
    Ok(())
}
