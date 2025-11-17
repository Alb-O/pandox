# pandox-markdown

Markdown to HTML converter powered by Pandoc, for use with Dioxus.

## Overview

This crate converts Markdown documents to HTML using `pandoc` and `dioxus_rsx_rosetta`.

## Usage

### As a Library

```rust
use pandox_markdown::MarkdownParser;
use std::path::Path;

// Parse a markdown file to HTML
let parser = MarkdownParser::new();
let html = parser.to_html_file(Path::new("public/blog-post.md"))?;

// Or from a string
let html = parser.to_html_string("# Hello\n\nWorld")?;
```

### In Dioxus

The `pandox-macros` crate calls into `MarkdownParser`, runs each block through `dx translate`, and embeds the generated RSX directly at compile time:

```rust
use pandox_macros::mdfile;

#[component]
fn BlogPost() -> Element {
    mdfile!("test/index.md", slug = "test")
}
```

## Dependencies

- `pandoc`: Rust bindings for Pandoc
- `pandoc_types`: Pandoc AST types (for advanced usage)
- `serde_json`: JSON serialization

### System Requirements

- **Pandoc**: Must be installed on your system
  - Installation: https://pandoc.org/installing.html
  - Included in flake.nix dev shell

## API

### `MarkdownParser`

```rust
let parser = MarkdownParser::new();

// Convert file to HTML
let html = parser.to_html_file(Path::new("example.md"))?;

// Convert string to HTML
let html = parser.to_html_string("# Hello")?;

// Advanced: Get Pandoc AST
let ast = parser.parse_file(Path::new("example.md"))?;
let ast = parser.parse_string("# Hello")?;
```

## Development

```bash
cargo test -p pandox-markdown
cargo build -p pandox-markdown
cargo run --example dioxus_example
```
