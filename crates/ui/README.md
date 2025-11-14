# UI

Shared components for the workspace, including block component factories for Pandoc-to-Dioxus conversion.

```
ui/
├─ src/
│  ├─ lib.rs    # Entrypoint
│  ├─ hero.rs   # Hero component
│  ├─ navbar.rs # Navbar component
│  ├─ block.rs  # Block component factories
```

## Block Components

Factory-pattern components for converting Pandoc AST blocks to Dioxus Elements using `dangerous_inner_html`.

### Simple rendering

```rust
use ui::blocks_from_components;

rsx! { {blocks_from_components(components)} }
```

### Typed rendering (with CSS classes)

```rust
use ui::typed_blocks;

rsx! { {typed_blocks(components)} }
```

Adds classes: `header-{level}`, `code-block`, `blockquote`, `bullet-list`, `table`, `figure`, etc.

### Custom rendering

```rust
use pandoc_types::definition::Block as PandocBlock;

match &component.block {
    PandocBlock::Header(level, _, _) => rsx! { /* custom */ },
    PandocBlock::CodeBlock(_, _) => rsx! { /* custom */ },
    _ => rsx! { div { dangerous_inner_html: "{component.html}" } }
}
```

### All 14 Pandoc Block types

`Plain`, `Para`, `LineBlock`, `CodeBlock`, `RawBlock`, `BlockQuote`, `OrderedList`, `BulletList`, `DefinitionList`, `Header`, `HorizontalRule`, `Table`, `Figure`, `Div`, `Null`

See `examples/typed_blocks.rs` for demo.

## Dependencies

Shared crate - no platform-specific deps. Platform deps go in [web](../web/Cargo.toml), [desktop](../desktop/Cargo.toml), [mobile](../mobile/Cargo.toml) crates.
