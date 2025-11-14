#![warn(
	missing_docs,
	missing_debug_implementations,
	missing_copy_implementations,
	trivial_bounds,
	unused_extern_crates,
	unused_results
)]

//! Pandox Parser - Markdown to HTML converter using Pandoc
//!
//! This crate provides functionality to parse Markdown files using Pandoc
//! and convert them to HTML for use with Dioxus's `dangerous_inner_html`.
//!
//! # Example
//!
//! ```no_run
//! use parser::MarkdownParser;
//! use std::path::Path;
//!
//! let parser = MarkdownParser::new();
//! parser.to_html_file(
//!     Path::new("content/test.md"),
//!     Path::new("dist/test.html"),
//!     "assets",
//!     None
//! ).unwrap();
//! ```

mod parser;

// Re-export pandoc_types for advanced usage
pub use pandoc_types::definition::{Block, Inline, Pandoc as PandocAst};
pub use parser::{BlockComponent, MarkdownParser};
