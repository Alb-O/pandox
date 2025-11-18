//! Pandox Parser - Markdown to HTML converter using Pandoc
//!
//! This crate provides functionality to parse Markdown files using Pandoc
//! and convert them to HTML for feeding into the `dx translate` pipeline that produces RSX.
//!
//! # Example
//!
//! ```no_run
//! use pandox_markdown::MarkdownParser;
//! use std::path::Path;
//!
//! let parser = MarkdownParser::new();
//! parser.to_html_file(
//!     Path::new("modules/test.md"),
//!     Path::new("dist/test.html"),
//!     "assets",
//!     None
//! ).unwrap();
//! ```

mod parser;

// Re-export pandoc_types for advanced usage
pub use pandoc_types::definition::{Block, Inline, Pandoc as PandocAst};
pub use parser::{BlockComponent, MarkdownParser};
