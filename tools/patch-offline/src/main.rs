//! Patches Trunk's dist output for file:// protocol compatibility.
//!
//! This tool transforms a Trunk-built web app to work when opened directly
//! via file:// protocol (double-clicking index.html), enabling offline use.
//!
//! Transformations:
//! 1. Embeds WASM as base64 in the JS file (fetch doesn't work on file://)
//! 2. Patches JS to use embedded bytes instead of fetching WASM
//! 3. Removes ES module syntax from HTML (modules don't work on file://)
//! 4. Removes crossorigin attributes (CORS fails on file://)
//! 5. Removes modulepreload/preload links

use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, ExitCode};

use base64::Engine;
use regex::Regex;

fn main() -> ExitCode {
	println!("Building with resources/offline.html (no-modules target)...");

	// Run trunk build with:
	// - BEZEL_OFFLINE=1 to generate relative asset paths
	// - resources/offline.html which uses data-bindgen-target="no-modules"
	let status = Command::new("trunk")
		.args(["build", "resources/offline.html"])
		.env("BEZEL_OFFLINE", "1")
		.status()
		.unwrap_or_else(|e| {
			eprintln!("Failed to execute trunk build: {e}");
			std::process::exit(1);
		});

	if !status.success() {
		eprintln!("Trunk build failed");
		return ExitCode::FAILURE;
	}

	let dist_dir = std::env::args()
		.nth(1)
		.unwrap_or_else(|| "dist".to_string());
	let dist_path = PathBuf::from(&dist_dir);

	if let Err(e) = patch_offline(&dist_path) {
		eprintln!("Error: {e}");
		return ExitCode::FAILURE;
	}

	ExitCode::SUCCESS
}

fn patch_offline(dist_dir: &Path) -> Result<(), String> {
	if !dist_dir.is_dir() {
		return Err(format!(
			"dist directory '{}' not found. Run 'trunk build' first.",
			dist_dir.display()
		));
	}

	let index_path = dist_dir.join("index.html");
	if !index_path.exists() {
		return Err(format!("index.html not found in '{}'", dist_dir.display()));
	}

	// Find the JS and WASM files (they have hashes in filenames)
	let js_file = find_file(dist_dir, "bezel-", ".js", Some("_bg"))?;
	let wasm_file = find_file(dist_dir, "bezel-", "_bg.wasm", None)?;

	println!("Patching for offline/file:// use...");
	println!("  JS:   {}", js_file.display());
	println!("  WASM: {}", wasm_file.display());

	// Patch JS file: embed WASM as base64
	patch_js_file(&js_file, &wasm_file)?;

	// Patch HTML file
	patch_html_file(&index_path, &js_file)?;

	// Remove WASM file (now embedded in JS)
	fs::remove_file(&wasm_file).map_err(|e| format!("Failed to remove WASM file: {e}"))?;
	println!(
		"Removed {} (now embedded in JS)",
		wasm_file.file_name().unwrap().to_string_lossy()
	);

	println!("\nOffline bundle ready!");
	println!(
		"Distribute: index.html, {}, and assets/ folder.",
		js_file.file_name().unwrap().to_string_lossy()
	);

	Ok(())
}

fn find_file(
	dir: &Path,
	prefix: &str,
	suffix: &str,
	exclude: Option<&str>,
) -> Result<PathBuf, String> {
	let entries = fs::read_dir(dir).map_err(|e| format!("Failed to read directory: {e}"))?;

	for entry in entries.flatten() {
		let name = entry.file_name();
		let name_str = name.to_string_lossy();
		if name_str.starts_with(prefix) && name_str.ends_with(suffix) {
			if let Some(exc) = exclude {
				if name_str.contains(exc) {
					continue;
				}
			}
			return Ok(entry.path());
		}
	}

	Err(format!(
		"Could not find file matching {prefix}*{suffix} in {}",
		dir.display()
	))
}

fn patch_js_file(js_path: &Path, wasm_path: &Path) -> Result<(), String> {
	let mut js_content =
		fs::read_to_string(js_path).map_err(|e| format!("Failed to read JS file: {e}"))?;

	let wasm_bytes = fs::read(wasm_path).map_err(|e| format!("Failed to read WASM file: {e}"))?;

	let wasm_base64 = base64::engine::general_purpose::STANDARD.encode(&wasm_bytes);
	let wasm_size_mb = wasm_bytes.len() as f64 / (1024.0 * 1024.0);

	// Create decoder snippet that decodes base64 to Uint8Array
	let decoder_snippet = format!(
		r#"
// Embedded WASM (base64) for file:// protocol support
const __offlineWasmBytes = (function() {{
  const binary = atob('{wasm_base64}');
  const bytes = new Uint8Array(binary.length);
  for (let i = 0; i < binary.length; i++) {{
    bytes[i] = binary.charCodeAt(i);
  }}
  return bytes;
}})();
"#
	);

	// Insert decoder at the start of the IIFE (after the opening)
	js_content = js_content.replacen(
		"let wasm_bindgen;\n(function() {",
		&format!("let wasm_bindgen;\n(function() {{{decoder_snippet}"),
		1,
	);

	// Patch the default WASM path resolution to use embedded bytes
	// The no-modules JS has: module_or_path = script_src.replace(/\.js$/, '_bg.wasm');
	let wasm_path_re = Regex::new(
        r"if \(typeof module_or_path === 'undefined' && typeof script_src !== 'undefined'\) \{\s*module_or_path = script_src\.replace\(/\\\.js\$/, '_bg\.wasm'\);\s*\}"
    ).unwrap();

	js_content = wasm_path_re
		.replace(
			&js_content,
			"if (typeof module_or_path === 'undefined') { module_or_path = __offlineWasmBytes; }",
		)
		.to_string();

	fs::write(js_path, js_content).map_err(|e| format!("Failed to write JS file: {e}"))?;

	println!(
		"Patched {} (embedded {:.1}MB WASM)",
		js_path.file_name().unwrap().to_string_lossy(),
		wasm_size_mb
	);

	Ok(())
}

fn patch_html_file(html_path: &Path, js_path: &Path) -> Result<(), String> {
	let mut html =
		fs::read_to_string(html_path).map_err(|e| format!("Failed to read HTML file: {e}"))?;

	let js_filename = js_path.file_name().unwrap().to_string_lossy();

	// Remove the ES module script block entirely
	let module_re = Regex::new(r#"<script type="module">.*?</script>"#).unwrap();
	html = module_re.replace(&html, "").to_string();

	// Remove modulepreload and preload links (they cause errors on file://)
	let modulepreload_re = Regex::new(r#"<link rel="modulepreload"[^>]*>"#).unwrap();
	let preload_re = Regex::new(r#"<link rel="preload"[^>]*>"#).unwrap();
	html = modulepreload_re.replace_all(&html, "").to_string();
	html = preload_re.replace_all(&html, "").to_string();

	// Remove crossorigin attributes
	let crossorigin_re = Regex::new(r#"\s+crossorigin(?:="[^"]*")?"#).unwrap();
	html = crossorigin_re.replace_all(&html, "").to_string();

	// Add script tags for external JS
	let loader_script = format!(
		r#"
<script src="./{js_filename}"></script>
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
"#
	);

	// Insert before </head>
	html = html.replace("</head>", &format!("{loader_script}</head>"));

	fs::write(html_path, html).map_err(|e| format!("Failed to write HTML file: {e}"))?;

	println!("Patched index.html");

	Ok(())
}
