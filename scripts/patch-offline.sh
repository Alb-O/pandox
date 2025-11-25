#!/usr/bin/env bash
# Builds and patches for file:// protocol compatibility (offline HTML).
# Creates a self-contained bundle that works when double-clicking index.html.
#
# Usage: ./scripts/patch-offline.sh [dist-dir]
#
# This script:
# 1. Builds using resources/offline.html (no-modules wasm-bindgen target)
# 2. Patches the JS file to embed WASM as base64 (fetch doesn't work on file://)
# 3. Removes ES module syntax from HTML (modules don't work on file://)
# 4. Removes crossorigin attributes (CORS fails on file://)
# 5. Removes modulepreload/preload links

set -euo pipefail

DIST_DIR="${1:-dist}"

# Build using the offline index file (no-modules target)
echo "Building with resources/offline.html (no-modules target)..."
trunk build --config Trunk.toml resources/offline.html

if [[ ! -d "$DIST_DIR" ]]; then
  echo "Error: dist directory '$DIST_DIR' not found. Build failed." >&2
  exit 1
fi

INDEX="$DIST_DIR/index.html"
if [[ ! -f "$INDEX" ]]; then
  echo "Error: index.html not found in '$DIST_DIR'" >&2
  exit 1
fi

# Find the JS and WASM files (they have hashes in filenames)
JS_FILE=$(find "$DIST_DIR" -maxdepth 1 -name "bezel-*.js" ! -name "*_bg*" -printf "%f\n" | head -1)
WASM_FILE=$(find "$DIST_DIR" -maxdepth 1 -name "bezel-*_bg.wasm" -printf "%f\n" | head -1)

if [[ -z "$JS_FILE" || -z "$WASM_FILE" ]]; then
  echo "Error: Could not find JS or WASM files in '$DIST_DIR'" >&2
  exit 1
fi

echo "Patching for offline/file:// use..."
echo "  JS:   $JS_FILE"
echo "  WASM: $WASM_FILE"

# Patch JS file to embed WASM as base64, and patch HTML
python3 - "$DIST_DIR" "$JS_FILE" "$WASM_FILE" << 'PYTHON_SCRIPT'
import sys
import re
import base64
import os

dist_dir = sys.argv[1]
js_file = sys.argv[2]
wasm_file = sys.argv[3]

index_path = os.path.join(dist_dir, "index.html")
js_path = os.path.join(dist_dir, js_file)
wasm_path = os.path.join(dist_dir, wasm_file)

# --- Patch JS file: embed WASM as base64 ---
with open(js_path, 'r') as f:
    js_content = f.read()

with open(wasm_path, 'rb') as f:
    wasm_bytes = f.read()

wasm_base64 = base64.b64encode(wasm_bytes).decode('ascii')
wasm_size_mb = len(wasm_bytes) / (1024 * 1024)

# Create decoder snippet that decodes base64 to Uint8Array
decoder_snippet = f'''
// Embedded WASM (base64) for file:// protocol support
const __offlineWasmBytes = (function() {{
  const binary = atob('{wasm_base64}');
  const bytes = new Uint8Array(binary.length);
  for (let i = 0; i < binary.length; i++) {{
    bytes[i] = binary.charCodeAt(i);
  }}
  return bytes;
}})();
'''

# Insert decoder at the start of the IIFE (after the opening)
js_content = js_content.replace(
    'let wasm_bindgen;\n(function() {',
    'let wasm_bindgen;\n(function() {' + decoder_snippet,
    1
)

# Patch the default WASM path resolution to use embedded bytes
# The no-modules JS has: module_or_path = script_src.replace(/\.js$/, '_bg.wasm');
js_content = re.sub(
    r"if \(typeof module_or_path === 'undefined' && typeof script_src !== 'undefined'\) \{\s*module_or_path = script_src\.replace\(/\\\.js\$/, '_bg\.wasm'\);\s*\}",
    "if (typeof module_or_path === 'undefined') { module_or_path = __offlineWasmBytes; }",
    js_content
)

with open(js_path, 'w') as f:
    f.write(js_content)

print(f"Patched {js_file} (embedded {wasm_size_mb:.1f}MB WASM)")

# --- Patch HTML file ---
with open(index_path, 'r') as f:
    html = f.read()

# Remove the ES module script block entirely
html = re.sub(
    r'<script type="module">.*?</script>',
    '',
    html,
    flags=re.DOTALL
)

# Remove modulepreload and preload links (they cause errors on file://)
html = re.sub(r'<link rel="modulepreload"[^>]*>', '', html)
html = re.sub(r'<link rel="preload"[^>]*>', '', html)

# Remove crossorigin attributes
html = re.sub(r'\s+crossorigin(?:="[^"]*")?', '', html)

# Add script tags for external JS
loader_script = f'''
<script src="./{js_file}"></script>
<script>
  // Initialize WASM (uses embedded bytes in patched JS)
  wasm_bindgen().then(function(wasm) {{
    console.log('WASM initialized (file:// mode)');
    window.wasmBindings = wasm_bindgen;
    dispatchEvent(new CustomEvent("TrunkApplicationStarted", {{detail: {{wasm}}}}));
  }}).catch(function(err) {{
    console.error('Failed to initialize WASM:', err);
  }});
</script>
'''

# Insert before </head>
html = html.replace('</head>', loader_script + '\n</head>')

with open(index_path, 'w') as f:
    f.write(html)

print(f"Patched index.html")
PYTHON_SCRIPT

echo "Distribute: index.html, $JS_FILE, and assets/ folder."
echo "Removing $WASM_FILE as it's now embedded."
rm -f "$DIST_DIR/$WASM_FILE"
