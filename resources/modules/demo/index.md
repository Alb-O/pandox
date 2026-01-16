---
title: "Overview"
category: "Demo"
order: 1
---

# Markdown Demo

This documentation showcases the markdown features supported by Bezel.

## Features

Bezel uses [Pandoc](https://pandoc.org) for markdown processing with extensions for:

- **Text formatting** - Bold, italic, strikethrough, inline code
- **Lists** - Ordered, unordered, task lists, definition lists
- **Media** - Links, images, local assets
- **Code blocks** - Syntax highlighting for multiple languages
- **Tables** - With column alignment
- **Blockquotes** - With nesting support
- **Advanced** - Math (MathJax), footnotes, HTML embedding

## Callout Blocks

Custom callout blocks using fenced divs:

::: {.callout .tip}
Use the sidebar to navigate between demo pages.
:::

## Offline Support

Bezel supports offline distribution via `file://` protocol. Build with:

```sh
trunk build --release
cargo run -p patch-offline -- dist
```
