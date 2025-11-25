use log::trace;

use std::io::Error;
use std::path::{Path, PathBuf};
use std::{env, fs};

use super::parse::StyleSheet;

/// Creates a theme in $HOME/.themes/, importing `default` as the fallback.
///
/// # Returns
/// * `Ok(StyleSheet)` — If the directory is created successfully, the file is
///   written, and the stylesheet parses correctly.
/// * `Err(Error)` — If theme creation fails, the theme already exists,
///   writing to disk fails, or reading the theme stylesheet fails.
///
/// # Errors and Panics
/// * **Panics** if `default` is a relative path.
/// * Returns `AlreadyExists` if the target theme already exists.
/// * Returns I/O errors directly from underlying filesystem operations.
pub fn create_as_edit(name: String, default: PathBuf) -> Result<StyleSheet, Error> {
    if default.is_relative() {
        panic!("default theme path cannot be relative");
    }

    let mut file_path = env::home_dir().expect("Failed to find user's home directory.");
    file_path.push(".themes/".to_string() + name.as_str());
    if fs::exists(&file_path).unwrap_or(false) {
        return Err(Error::new(
            std::io::ErrorKind::AlreadyExists,
            "Theme already exists",
        ));
    }
    trace!(
        "Theme existence check passed. Theme with name \"{}\" does not yet exist.",
        name
    );
    file_path.push("cinnamon");

    trace!(
        "Creating theme directory at path {:?}.",
        file_path.as_os_str()
    );
    if let Err(err) = fs::create_dir_all(&file_path) {
        return Err(err);
    }
    file_path.push(Path::new("cinnamon.css"));

    trace!("Writing cinnamon.css stylesheet for theme \"{}\".", name);
    if let Err(err) = fs::write(
        &file_path,
        format!(
            "@import url(\"{}\");\n",
            default.as_os_str().to_str().unwrap()
        ),
    ) {
        return Err(err);
    }

    trace!(
        "Reading back cinnamon.css stylesheet for theme \"{}\".",
        name
    );
    let result = fs::read_to_string(&file_path);

    result.map(|raw| StyleSheet::parse(file_path.to_path_buf(), raw))
}

pub async fn open_existing(name: String) {
    todo!()
}
