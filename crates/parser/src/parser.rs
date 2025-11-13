use std::path::{Path, PathBuf};
use log::debug;

use pandoc::{OutputFormat, OutputKind, Pandoc};

/// Find project root by searching upwards for Cargo.toml
/// Falls back to current directory if not found
fn find_project_root() -> PathBuf {
	let mut current = std::env::current_dir()
		.expect("Failed to get current directory");

	let original = current.clone();
	loop {
		if current.join("Cargo.toml").exists() {
			return current;
		}

		if !current.pop() {
			// No Cargo.toml found, return original current dir
			return original;
		}
	}
}

/// Main parser struct for converting markdown to Pandoc AST
#[derive(Default)]
pub struct MarkdownParser;

impl MarkdownParser {
	pub fn new() -> Self {
		Self
	}

	/// Convert markdown file to HTML string
	pub fn to_html_file(&self, input: &Path, output: &Path, asset_slug: &str, project_root: Option<&Path>) -> Pandoc {
		let mut pandoc = Pandoc::new();

		// Determine project root: use provided, or find it, or use current dir
		let root = project_root
			.map(|p| p.to_path_buf())
			.unwrap_or_else(find_project_root);

		debug!("Using project root: {:?}", root);

		// Make paths absolute relative to project root
		let input_abs = if input.is_absolute() {
			input.to_path_buf()
		} else {
			root.join(input)
		};

		let output_abs = if output.is_absolute() {
			output.to_path_buf()
		} else {
			root.join(output)
		};

		if !input_abs.exists() {
			panic!("Input file not found: {} (resolved from {})", input_abs.display(), input.display());
		}

		// Create output dir
		if let Some(parent) = output_abs.parent() {
			std::fs::create_dir_all(parent)
				.unwrap_or_else(|_| panic!("Failed to create output dir: {}", parent.display()));
		}

		let output_asset_dir = output_abs.parent().unwrap().join(asset_slug);

		debug!(
			"Converting markdown file {:?} to HTML file {:?} with assets in {:?}",
			input_abs,
			output_abs,
			output_asset_dir
		);

		// Change CWD to output so ExtractMedia works with relative path
		std::fs::create_dir_all(&output_asset_dir).unwrap();
		std::env::set_current_dir(output_abs.parent().unwrap()).unwrap();

		pandoc.add_options(&[
			pandoc::PandocOption::ExtractMedia(asset_slug.into()),
			pandoc::PandocOption::ResourcePath(vec![input_abs.parent().unwrap().to_path_buf()]),
			pandoc::PandocOption::NumberSections,
		])
		.set_input(pandoc::InputKind::Files(vec![input_abs.clone()]))
		.set_input_format(
			pandoc::InputFormat::Markdown,
			vec![
				pandoc::MarkdownExtension::RebaseRelativePaths,
				pandoc::MarkdownExtension::Smart,
			],
		)
		.set_output(OutputKind::File(output_abs.clone()))
		.set_output_format(OutputFormat::Html, vec![]);

		pandoc
	}
}
