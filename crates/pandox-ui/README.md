# pandox-ui

Shared components for the workspace, including block component factories for Pandoc-to-Dioxus conversion.

```
pandox-ui/
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
use pandox_ui::Block;

rsx! { Block { html: component.html.clone() } }
```

### Typed rendering (with CSS classes/manifest)

```rust
use pandox_ui::{TypedBlock, BLOCK_MANIFEST};

for component in components {
    TypedBlock { component: component.clone() }
}
```

Adds classes: `header-{level}`, `code-block`, `blockquote`, `bullet-list`, `table`, `figure`, etc.

### Custom rendering / manifest extension

```rust
use pandox_markdown::BlockComponent;
use pandoc_types::definition::Block as PandocBlock;
use pandox_ui::{BLOCK_MANIFEST, BlockRenderer, render_from_manifest};

fn code_with_icon(component: &BlockComponent) -> Option<Element> {
    matches!(&component.block, PandocBlock::CodeBlock(_, _)).then(|| {
        let html = component.html.clone();
        rsx! {
            div { class: "code-block decorated", dangerous_inner_html: "{html}" }
        }
    })
}

let manifest: Vec<BlockRenderer> =
    std::iter::once(code_with_icon as BlockRenderer)
        .chain(BLOCK_MANIFEST.iter().copied())
        .collect();

render_from_manifest(&component, &manifest)
    .unwrap_or_else(|| /* fallback */);
```

### All 14 Pandoc Block types

`Plain`, `Para`, `LineBlock`, `CodeBlock`, `RawBlock`, `BlockQuote`, `OrderedList`, `BulletList`, `DefinitionList`, `Header`, `HorizontalRule`, `Table`, `Figure`, `Div`, `Null`

See `examples/typed_blocks.rs` for demo.

## Dependencies

Shared crate - no platform-specific deps. Platform deps go in [pandox-web](../pandox-web/Cargo.toml) or any other app crates.
