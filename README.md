# Bezel

Leptos CSR app with markdown content rendering and offline support.

## Development

```sh
trunk serve --public-url /
```

Serves at `http://localhost:8080`. The `--public-url /` flag is required for asset loading on nested routes.

## Build

```sh
trunk build --release
```

Output goes to `dist/`.

## Offline Build

For file:// protocol distribution (no server required):

```sh
trunk build --release
cargo run -p patch-offline -- dist
```
