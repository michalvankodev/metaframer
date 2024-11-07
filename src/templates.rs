use anyhow::Context;
use dirs::{self, config_dir};
use handlebars::{self, Handlebars};
use log::debug;
use std::fs;
use std::path::{Path, PathBuf};

pub fn get_templates_path(template_name: &str) -> PathBuf {
    let config_dir = config_dir().unwrap();
    let dest_path = Path::new(&config_dir)
        .join("metaframer/templates/")
        .join(template_name);
    dest_path
}

pub fn init_templates_if_needed() -> Result<(), anyhow::Error> {
    let default_template_path = get_templates_path("default");
    if default_template_path.exists() {
        return Ok(());
    }
    copy_default_template()?;
    Ok(())
}

/**
 * Exctracts default templates overwriting if any change was made by user
 */
pub fn copy_default_template() -> Result<(), anyhow::Error> {
    // Create the destination directory if it doesn't exist
    let default_template_path = get_templates_path("default");
    fs::create_dir_all(&default_template_path)?;

    // Copy the template files to the destination directory
    let default_template_files = [
        ("main.svg", include_str!("../templates/default/main.svg")),
        (
            "iso-icon.svg",
            include_str!("../templates/default/iso-icon.svg"),
        ),
        (
            "camera-icon.svg",
            include_str!("../templates/default/camera-icon.svg"),
        ),
        (
            "aperture-icon.svg",
            include_str!("../templates/default/aperture-icon.svg"),
        ),
        (
            "focal-length-icon.svg",
            include_str!("../templates/default/focal-length-icon.svg"),
        ),
        (
            "shutter-speed-icon.svg",
            include_str!("../templates/default/shutter-speed-icon.svg"),
        ),
    ];

    for (name, content) in default_template_files {
        let path = default_template_path.join(name);
        fs::write(&path, content)?;
    }
    Ok(())
}

pub fn register_templates(
    template_name: &str,
    handlebars: &mut Handlebars,
) -> Result<(), anyhow::Error> {
    let templates_path = get_templates_path(template_name);
    debug!("{:?} templates path", templates_path);
    handlebars
        .register_template_file("main", templates_path.join("main.svg"))
        .with_context(|| {
            format!(
                "could not read template file`{:?}` in `{:?}`",
                "main.svg".to_string(),
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
    Ok(())
}
