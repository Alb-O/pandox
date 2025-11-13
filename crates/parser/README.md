# Parser

Markdown to HTML converter powered by Pandoc, for use with Dioxus.

## Overview

This crate converts Markdown documents to HTML using Pandoc. The HTML can then be rendered in Dioxus applications using `dangerous_inner_html`.

## Usage

### As a Library

```rust
use parser::MarkdownParser;
use std::path::Path;

// Parse a markdown file to HTML
let parser = MarkdownParser::new();
let html = parser.to_html_file(Path::new("content/blog-post.md"))?;

// Or from a string
let html = parser.to_html_string("# Hello\n\nWorld")?;
```

### In Dioxus

```rust
use parser::MarkdownParser;

fn BlogPost() -> Element {
    let parser = MarkdownParser::new();
    let html = parser.to_html_file(Path::new("post.md")).unwrap();

    rsx! {
        div {
            class: "markdown-content",
            dangerous_inner_html: "{html}"
        }
    }
}
```

### As a CLI Tool

```bash
# Run the demo
cargo run -p parser

# Build the binary
cargo build -p parser --release
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
cargo test -p parser
cargo build -p parser
cargo run --example dioxus_example
```
