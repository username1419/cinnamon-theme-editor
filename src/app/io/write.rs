use dioxus::prelude::debug;

use crate::app::io::parse::StyleSheet;
use std::{fs::write, io::Error};

pub fn save_theme(stylesheet: &StyleSheet) -> Result<(), Error> {
    let source = stylesheet.get_source();
    let out = stylesheet.to_save_string();

    debug!("Writing out {} to {:?}", out, source);
    write(source, out)?;
    debug!("Write out successful");
    Ok(())
}

pub fn export_theme(stylesheet: &StyleSheet) -> Result<(), Error> {
    let source = stylesheet.get_source();
    let out = stylesheet.to_export_string();

    debug!("Writing out {} to {:?}", out, source);
    write(source, out)?;
    debug!("Write out successful");
    Ok(())
}
