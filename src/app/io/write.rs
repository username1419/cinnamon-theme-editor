//! Persisting edited stylesheets to disk.

use dioxus::prelude::debug;

use crate::app::io::parse::StyleSheet;
use std::{fs::write, io::Error};

/// Writes the stylesheet to its [`StyleSheet::get_source`] path using the in-editor format
/// (selectors unchanged; includes `@import` when present).
pub fn save_theme(stylesheet: &StyleSheet) -> Result<(), Error> {
    let source = stylesheet.get_source();
    let out = stylesheet.to_save_string();

    debug!("Writing out {} to {:?}", out, source);
    write(source, out)?;
    debug!("Write out successful");
    Ok(())
}

/// Writes the stylesheet to its source path in export format: selectors are converted
/// via [`crate::app::io::parser::selector::Selector::to_export_safe`] so type/id selectors
/// are restored after webview-safe editing.
pub fn export_theme(stylesheet: &StyleSheet) -> Result<(), Error> {
    let source = stylesheet.get_source();
    let out = stylesheet.to_export_string();

    debug!("Writing out {} to {:?}", out, source);
    write(source, out)?;
    debug!("Write out successful");
    Ok(())
}
