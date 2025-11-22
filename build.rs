use std::path::{Path, PathBuf};
use std::{env, fs};

use pandoc::{
	InputFormat, InputKind, MarkdownExtension, OutputFormat, OutputKind, PandocOption, PandocOutput,
};
use walkdir::WalkDir;

fn main() {
	println!("cargo:rerun-if-changed=modules");

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

	for entry in WalkDir::new("modules")
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
		let media_dir = generated_assets_root.join(slug);

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

		let html = match pandoc.execute().expect("pandoc") {
			PandocOutput::ToBuffer(html) => {
				let media_prefix = media_dir.to_string_lossy().replace('\\', "/") + "/";
				html.replace(&media_prefix, &format!("/assets/{slug}/"))
			}
			PandocOutput::ToBufferRaw(bytes) => String::from_utf8(bytes).expect("utf8 html"),
			PandocOutput::ToFile(path) => fs::read_to_string(path).expect("read html"),
		};

		let html_path = out_dir.join("modules").join(slug).join("index.html");
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
