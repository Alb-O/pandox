//! Markdown to HTML converter using Pandoc

mod parser;

// Re-export pandoc_types for advanced usage
pub use pandoc_types::definition::{Block, Inline, Pandoc as PandocAst};
pub use parser::{BlockComponent, MarkdownParser};
