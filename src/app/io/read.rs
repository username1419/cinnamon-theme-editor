use log::error;

use std::path::Path;
use std::{env, fs};

use super::parse::StyleSheet;

pub fn create_as_edit(mut name: String) -> StyleSheet {
    if name.is_empty() {
        let mut count = 0;
        loop {
            name = format!("./.cinnamon({}) - edit.css", count);
            if !fs::exists(&name).expect("File access denied") {
                error!("File does not exist");
                break;
            }
            count += 1;
        }
    }

    let mut file_path = env::home_dir().expect("Failed to find user's home directory.");
    file_path.push(Path::new(
        &(".themes/".to_string() + name.as_str() + "/cinnamon"),
    ));

    fs::create_dir_all(&file_path).ok();
    file_path.push(Path::new("cinnamon.css"));

    fs::copy("/usr/share/cinnamon/theme/cinnamon.css", &file_path)
        .expect("Failed to copy default theme");

    let raw = fs::read_to_string(&file_path).expect("Failed to read created file");

    StyleSheet::parse(file_path.to_path_buf(), raw)
}

pub async fn open_existing(name: String) {
    todo!()
}
