use dioxus::prelude::debug;

use crate::app::io::parse::StyleSheet;
use std::{fs::write, io::Error};

pub fn save_theme(stylesheet: &StyleSheet) -> Result<(), Error> {
    let source = stylesheet.get_source();
    let out = stylesheet.to_save_string();

    debug!("Written out {} to {:?}", out, source);
    write(source, out)?;
    Ok(())
}

pub fn export_theme(stylesheet: &StyleSheet) -> Result<(), Error> {
    let source = stylesheet.get_source();
    let out = stylesheet.to_export_string();

    debug!("Written out {} to {:?}", out, source);
    write(source, out)?;
    Ok(())
}
