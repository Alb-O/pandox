# SVG Icon System for Bezel (Tabler Icons)

## Overview

Type-safe SVG icon system that:
1. Downloads only needed Tabler Icons at build time
2. Caches them locally in `bezel/icons/` (version controlled)
3. Generates Rust enum for Leptos components
4. Supports icon references in markdown via Pandoc filter

```
┌─────────────────────────────────────────────────────────────────┐
│  SOURCES                                                        │
│  ├── icons.toml (explicit list for Rust code)                  │
│  └── Markdown files (scan for []{.icon .name} spans)           │
└──────────────────────────┬──────────────────────────────────────┘
                           ▼
┌─────────────────────────────────────────────────────────────────┐
│  BUILD (build.rs)                                               │
│  1. Collect needed icons from both sources                      │
│  2. Download missing → bezel/icons/{name}.svg                   │
│  3. Generate $OUT_DIR/icons.rs (enum + svg strings)             │
│  4. Pandoc filter: convert icon spans to inline SVG             │
└──────────────────────────┬──────────────────────────────────────┘
                           ▼
┌─────────────────────────────────────────────────────────────────┐
│  OUTPUT                                                         │
│  ├── IconKind enum (Rust code usage)                           │
│  └── Inline SVG in HTML (markdown content)                      │
└─────────────────────────────────────────────────────────────────┘
```

## Icon Cache: `bezel/icons/`

Local cache in project directory (version controlled):
```
bezel/
└── icons/
    ├── .version          # "v3.28.1" - invalidates cache on change
    ├── check.svg
    ├── alert-triangle.svg
    └── info-circle.svg
```

**Why in-repo?**
- Reproducible builds without network
- Git tracks exactly which icons are used
- CI doesn't need to download
- Only used icons, not all 5000

## Icon Sources

### 1. Explicit manifest (icons.toml)

For icons used in Rust code:

```toml
[config]
version = "v3.28.1"
style = "outline"  # or "filled"

[icons]
check = {}
alert-triangle = {}
info-circle = {}
x = {}
chevron-right = {}
menu-2 = {}
```

### 2. Markdown icon syntax

Using Pandoc's BracketedSpans (already enabled):

```markdown
Click the []{.icon .check} button to confirm.

Status: []{.icon .alert-triangle} Warning detected.
```

Build script scans markdown for `[]{.icon .NAME}` patterns and adds to download list.

## Generated Artifacts

### 1. Rust enum ($OUT_DIR/icons.rs)

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum IconKind {
    Check,
    AlertTriangle,
    InfoCircle,
    X,
    ChevronRight,
    Menu2,
}

impl IconKind {
    pub const fn svg(&self) -> &'static str {
        match self {
            Self::Check => "<svg>...</svg>",
            // ...
        }
    }

    pub const fn name(&self) -> &'static str {
        match self {
            Self::Check => "check",
            // ...
        }
    }
}
```

### 2. Icon component (src/icons.rs)

```rust
use leptos::prelude::*;

include!(concat!(env!("OUT_DIR"), "/icons.rs"));

#[component]
pub fn Icon(
    icon: IconKind,
    #[prop(optional, into)] class: String,
    #[prop(optional, default = 24)] size: u32,
) -> impl IntoView {
    view! {
        <span
            class=format!("icon {}", class)
            style=format!("width: {}px; height: {}px;", size, size)
            inner_html=icon.svg()
        />
    }
}
```

### 3. Inline SVG in markdown HTML

Pandoc filter converts:
```markdown
[]{.icon .check}
```
To:
```html
<span class="icon" aria-label="check"><svg>...</svg></span>
```

## Files to Create/Modify

| File | Action | Purpose |
|------|--------|---------|
| `icons.toml` | Create | Manifest for Rust-used icons |
| `bezel/icons/` | Create | SVG cache directory |
| `build.rs` | Modify | Icon downloading, scanning, code gen, Pandoc filter |
| `Cargo.toml` | Modify | Add `toml`, `ureq` build-deps |
| `src/icons.rs` | Create | Include wrapper + Icon component |
| `src/lib.rs` | Modify | Add `mod icons` |

## Implementation Steps

### Step 1: Dependencies (Cargo.toml)

```toml
[build-dependencies]
toml = "0.8"
ureq = "2.10"
regex = "1"  # for scanning markdown
```

### Step 2: Create icons.toml

```toml
[config]
version = "v3.28.1"
style = "outline"

[icons]
# Callout icons
alert-triangle = {}
info-circle = {}
circle-check = {}
alert-octagon = {}
# UI icons
check = {}
x = {}
chevron-right = {}
chevron-down = {}
menu-2 = {}
search = {}
```

### Step 3: Extend build.rs

Add before markdown processing:

```rust
fn collect_icons_from_manifest() -> HashSet<String> { ... }
fn collect_icons_from_markdown(markdown: &str) -> HashSet<String> { ... }
fn download_icon(name: &str, version: &str, style: &str) -> String { ... }
fn generate_icons_rs(icons: &HashMap<String, String>) { ... }
```

Pandoc filter addition:
```rust
// In the existing pandoc.add_filter(), also handle icon spans
// Convert Span with classes ["icon", "name"] to RawInline HTML
```

### Step 4: Create src/icons.rs

Include generated enum + define Icon component.

### Step 5: Update src/lib.rs

```rust
mod icons;
pub use icons::{Icon, IconKind};
```

## Usage Examples

### In Rust/Leptos

```rust
use crate::{Icon, IconKind};

view! {
    <button>
        <Icon icon=IconKind::Check />
        " Save"
    </button>
}
```

### In Markdown

```markdown
::: {.callout .warning}
[]{.icon .alert-triangle} This action cannot be undone.
:::

Press []{.icon .check} to confirm or []{.icon .x} to cancel.
```

### Callout enhancement (optional)

Could auto-inject icons into callouts based on variant:
```markdown
::: {.callout .warning}
This action cannot be undone.
:::
```
Becomes HTML with icon automatically inserted.

## Verification

1. Add an icon to `icons.toml`, run `cargo build`
2. Check `bezel/icons/` has the SVG file
3. Check `target/.../out/icons.rs` has the enum variant
4. Use `<Icon icon=IconKind::Name />` in a component
5. Add `[]{.icon .name}` to markdown, verify HTML output
6. Build offline (`BEZEL_OFFLINE=1`) - should work without network
