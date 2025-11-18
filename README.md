# pandox-web

Single crate layout. Web target sits at repo root with supporting crates in `crates/`.

```
.
├─ assets/           # web-only assets bundled by manganis
├─ modules/          # markdown content + images consumed by mdfile!()
├─ src/              # router + views
├─ crates/           # shared helpers (ui, macros, markdown glue)
└─ Dioxus.toml       # dx serve config
```

## Dev

```
dx serve
```
