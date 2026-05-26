//! Theme I/O: read, parse, edit, and write Cinnamon CSS stylesheets.
//!
//! Themes live under `$HOME/.themes/<name>/`. The editor uses a separate process
//! `.cinnamon-edit.css` that may `@import` the original theme; [`parse::StyleSheet`]
//! holds the parsed rulesets, and [`read`] / [`mod@write`] handle filesystem operations.

pub mod parse;
pub mod parser;
pub mod read;
mod test;
pub mod write;
