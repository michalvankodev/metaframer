use anyhow::Result;
use assert_cmd::Command;
use predicates::str;

#[test]
fn file_doesnt_exist() -> Result<()> {
    let mut cmd = Command::cargo_bin("metaframer")?;

    cmd.arg("test/file/doesnt/exist");
    cmd.assert()
        .success()
        .stderr(str::contains("could not read file"));

    Ok(())
}

#[test]
fn file_is_not_valid() -> Result<()> {
    let mut cmd = Command::cargo_bin("metaframer")?;

    cmd.arg("tests/assets/plain.txt");
    cmd.assert()
        .success()
        .stderr(str::contains("is not a valid image"));

    Ok(())
}

#[test]
fn file_is_valid() -> Result<()> {
    let mut cmd = Command::cargo_bin("metaframer")?;

    cmd.arg("tests/assets/image.jpg");
    cmd.assert().success();

    Ok(())
}
