//! Build script that converts Markdown to HTML and extracts media for the CSR app.

use std::path::{Path, PathBuf};
use std::{env, fs};

use pandoc::{
	InputFormat, InputKind, MarkdownExtension, OutputFormat, OutputKind, PandocOption, PandocOutput,
};
use serde_json::Value;
use walkdir::WalkDir;

const MODULES_DIR: &str = "resources/modules";

/// Adjusts media URLs so pandoc hashes them via `--extract-media`.
/// Adds a redundant `../` segment to force pandoc to treat the path as non-original.
fn bump_media_path(url: &mut String) {
	if url.starts_with('#')
		|| url.starts_with("http://")
		|| url.starts_with("https://")
		|| url.starts_with("data:")
		|| url.contains("..")
	{
		return;
	}
	let bumped = match url.trim_start_matches("./").split_once('/') {
		Some((first, rest)) => format!("{first}/../{first}/{rest}"),
		None => format!("../{url}"),
	};
	*url = bumped;
}

/// Recursively rewrites image/link targets in a Pandoc JSON document.
fn rewrite_media_links(value: &mut Value) {
	match value {
		Value::Array(items) => {
			if items.first().and_then(Value::as_str) == Some("Image") {
				if let Some(Value::Array(target)) = items.get_mut(3) {
					if let Some(Value::String(url)) = target.get_mut(0) {
						bump_media_path(url);
					}
				}
			}
			for item in items {
				rewrite_media_links(item);
			}
		}
		Value::Object(map) => {
			if map.get("t").and_then(Value::as_str) == Some("Image") {
				if let Some(Value::Array(c)) = map.get_mut("c") {
					if let Some(Value::Array(target)) = c.get_mut(2) {
						if let Some(Value::String(url)) = target.get_mut(0) {
							bump_media_path(url);
						}
					}
				}
			}
			for (_, v) in map.iter_mut() {
				rewrite_media_links(v);
			}
		}
		_ => {}
	}
}

fn main() {
	println!("cargo:rerun-if-changed={MODULES_DIR}");
	println!("cargo:rerun-if-env-changed=BEZEL_OFFLINE");

	let out_dir = PathBuf::from(env::var("OUT_DIR").expect("OUT_DIR not set"));
	let target_dir =
		PathBuf::from(env::var("CARGO_TARGET_DIR").unwrap_or_else(|_| "target".into()));
	let generated_assets_root = target_dir.join("generated-assets");
	let generated = out_dir.join("content.rs");
	let mut generated_code = String::from(
		"pub struct Page { pub slug: &'static str, pub html: &'static str }\n\npub const PAGES: &[Page] = &[\n",
	);

	let _ = fs::remove_dir_all(&generated_assets_root);
	fs::create_dir_all(&generated_assets_root).expect("create generated assets root");

	for entry in WalkDir::new(MODULES_DIR)
		.into_iter()
		.filter_map(Result::ok)
		.filter(|e: &walkdir::DirEntry| {
			e.path().extension().and_then(|ext| ext.to_str()) == Some("md")
		}) {
		let path = entry.path();
		let module_dir = path.parent().unwrap_or(Path::new("."));
		let slug = module_dir
			.file_name()
			.and_then(|name| name.to_str())
			.expect("module folder name");
		// Extract media into a single pooled generated assets directory.
		// Pandoc will hash filenames based on file content to deduplicate and avoid collisions.
		let media_dir = generated_assets_root.clone();

		let markdown = fs::read_to_string(path).expect("read markdown");

		fs::create_dir_all(&media_dir).expect("create media dir");

		let mut pandoc = pandoc::new();
		pandoc.set_input(InputKind::Pipe(markdown));
		pandoc.add_option(PandocOption::ResourcePath(vec![module_dir.into()]));
		pandoc.set_input_format(
			InputFormat::Markdown,
			vec![
				MarkdownExtension::FencedDivs,
				MarkdownExtension::BracketedSpans,
				MarkdownExtension::FencedCodeAttributes,
				MarkdownExtension::ImplicitFigures,
				MarkdownExtension::RawHtml,
				MarkdownExtension::Footnotes,
				MarkdownExtension::TaskLists,
				MarkdownExtension::PipeTables,
				MarkdownExtension::Smart,
			],
		);
		pandoc.set_output_format(OutputFormat::Html, Vec::new());
		pandoc.set_output(OutputKind::Pipe);
		pandoc.add_option(PandocOption::MathJax(None));
		pandoc.add_option(PandocOption::ExtractMedia(media_dir.clone()));
		pandoc.add_filter(|json| {
			let mut doc: Value = serde_json::from_str(&json).expect("parse pandoc json");
			rewrite_media_links(&mut doc);
			serde_json::to_string(&doc).expect("serialize pandoc json")
		});

		let html = match pandoc.execute().expect("pandoc") {
			PandocOutput::ToBuffer(html) => {
				let media_prefix = media_dir.to_string_lossy().replace('\\', "/") + "/";
				// Use relative paths for offline builds (file:// protocol), absolute for server
				// Set BEZEL_OFFLINE=1 environment variable for offline builds
				let assets_prefix = if env::var("BEZEL_OFFLINE").is_ok() {
					"./assets/"
				} else {
					"/assets/"
				};
				html.replace(&media_prefix, assets_prefix)
			}
			PandocOutput::ToBufferRaw(bytes) => String::from_utf8(bytes).expect("utf8 html"),
			PandocOutput::ToFile(path) => fs::read_to_string(path).expect("read html"),
		};

		let html_path = out_dir.join(MODULES_DIR).join(slug).join("index.html");
		if let Some(parent) = html_path.parent() {
			fs::create_dir_all(parent).expect("create module dir");
		}
		fs::write(&html_path, html).expect("write html");

		let include_path = html_path.to_str().expect("html path").replace('\\', "/");
		generated_code.push_str(&format!(
			"    Page {{ slug: \"{slug}\", html: include_str!(r\"{include_path}\") }},\n"
		));
	}

	generated_code.push_str("];\n");
	fs::write(generated, generated_code).expect("write generated rust");
}
