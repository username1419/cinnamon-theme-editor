use std::result::Result::Ok;

use std::io::Error;
use std::path::{Path, PathBuf};
use std::{env, fs};

use dioxus::prelude::debug;

use super::parse::StyleSheet;

/// Copy a directory recursively, in a depth-first manner. The function operates so that the result
/// of the function is `to/dir_contents`, assuming directory `to` already exists.
fn copy_recursive(src: &Path, dst: &Path) -> Result<(), Error> {
    fs::create_dir_all(dst)?;

    // Stack holds (source_dir, destination_dir)
    let mut stack: Vec<(PathBuf, PathBuf)> = Vec::new();
    stack.push((src.to_path_buf(), dst.to_path_buf()));

    while let Some((current_src, current_dst)) = stack.pop() {
        for entry in fs::read_dir(&current_src)? {
            let entry = entry?;
            let file_type = entry.file_type()?;
            let src_path = entry.path();
            let dst_path = current_dst.join(entry.file_name());

            if file_type.is_dir() {
                fs::create_dir_all(&dst_path)?;
                stack.push((src_path, dst_path));
            } else {
                fs::copy(&src_path, &dst_path)?;
            }
        }
    }

    Ok(())
}

pub fn is_theme_exist(name: &str) -> Result<bool, Error> {
    let mut file_path = env::home_dir().expect("Failed to find user's home directory.");
    file_path.push(".themes/".to_string() + name);

    fs::exists(&file_path)
}

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
    let theme_exists = is_theme_exist(&name);
    if theme_exists.is_err() || theme_exists.as_ref().is_ok_and(|e| e.eq(&true)) {
        theme_exists?;
        return Err(Error::new(
            std::io::ErrorKind::AlreadyExists,
            "Theme name already exists.",
        ));
    }

    fs::create_dir(&file_path)?;
    debug!("Copying theme from {:?} to {:?}", default, file_path);
    copy_recursive(&default, &file_path)?;

    // NOTE: idk if i should remove the original css file or not
    file_path.push(".cinnamon-edit.css");
    fs::write(&file_path, format!("@import url({:?});", default))?;

    debug!(
        "Reading back {:?} stylesheet for theme \"{}\".",
        file_path, name
    );
    let result = fs::read_to_string(&file_path);
    debug!("Read content {:?} from {:?}", result, file_path);

    result.map(|raw| StyleSheet::parse(file_path.to_path_buf(), raw))
}

pub fn open_existing(mut file_path: PathBuf) -> Result<StyleSheet, Error> {
    let name = file_path
        .iter()
        .next_back()
        .unwrap_or_default()
        .to_str()
        .unwrap_or_default()
        .to_string();

    file_path.push(".cinnamon-edit.css");
    if !fs::exists(&file_path).unwrap_or(false) {
        let theme_file_path = PathBuf::from(format!(
            "{}/cinnamon/cinnamon.css",
            file_path.to_str().unwrap_or_default()
        ));
        match fs::exists(&theme_file_path) {
            Ok(exists) => {
                if !exists {
                    // honestly i have no idea how this ? works but eh
                    fs::copy(theme_file_path, &file_path)?;
                }
            }
            Err(err) => {
                return Err(err);
            }
        }
    }

    debug!("Reading {:?} stylesheet for theme \"{}\".", file_path, name);
    let result = fs::read_to_string(&file_path);
    debug!("Read content {:?} from {:?}", result, file_path);

    result.map(|raw| StyleSheet::parse(file_path.to_path_buf(), raw))
}
