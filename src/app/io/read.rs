use log::error;

use std::path::{Path, PathBuf};
use std::{env, fs};

use super::parse::StyleSheet;

/// Creates a theme in $HOME/.themes/, importing `default` as the fallback.
///
/// # Panic
/// The function panics if the `default` path provided is a relative path, or an error is
/// encountered in the process of creating theme.
pub fn create_as_edit(name: String, default: PathBuf) -> StyleSheet {
    if default.is_relative() {
        panic!("default theme path cannot be relative");
    }

    let mut file_path = env::home_dir().expect("Failed to find user's home directory.");
    file_path.push(Path::new(
        &(".themes/".to_string() + name.as_str() + "/cinnamon"),
    ));

    fs::create_dir_all(&file_path).ok();
    file_path.push(Path::new("cinnamon.css"));

    fs::write(
        &file_path,
        format!(
            "@import url(\"{}\");\n",
            default.as_os_str().to_str().unwrap()
        ),
    )
    .ok();

    let raw = fs::read_to_string(&file_path).expect("Failed to read created file");

    StyleSheet::parse(file_path.to_path_buf(), raw)
}

pub async fn open_existing(name: String) {
    todo!()
}
