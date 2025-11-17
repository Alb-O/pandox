use std::path::{Path, PathBuf};

use log::debug;
use pandoc::{OutputFormat, OutputKind, Pandoc, PandocOutput};
use pandoc_types::definition::{Block, Pandoc as PandocAst};
use with_dir::WithDir;

/// Markdown block with rendered HTML
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct BlockComponent {
	/// Rendered HTML for this block
	pub html: String,
	/// Pandoc AST block type
	pub block: Block,
}

fn find_project_root() -> PathBuf {
	let mut current = std::env::current_dir().expect("Failed to get current directory");
	let original = current.clone();
	loop {
		if current.join("Cargo.toml").exists() {
			return current;
		}
		if !current.pop() {
			return original;
		}
	}
}

fn abs_path(path: &Path, root: &Path) -> PathBuf {
	if path.is_absolute() {
		path.to_path_buf()
	} else {
		root.join(path)
	}
}

fn resource_path(path: &Path) -> PathBuf {
	path.parent().unwrap().to_path_buf()
}

fn markdown_pandoc(input_abs: &Path, output: Option<&Path>, asset_slug: Option<&str>) -> Pandoc {
	let mut pandoc = Pandoc::new();
	let mut opts = vec![
		pandoc::PandocOption::ResourcePath(vec![resource_path(input_abs)]),
		pandoc::PandocOption::NumberSections,
	];
	if let Some(slug) = asset_slug {
		opts.push(pandoc::PandocOption::ExtractMedia(slug.into()));
	}
	let _ = pandoc
		.add_options(&opts)
		.set_input(pandoc::InputKind::Files(vec![input_abs.to_path_buf()]))
		.set_input_format(
			pandoc::InputFormat::Markdown,
			vec![
				pandoc::MarkdownExtension::RebaseRelativePaths,
				pandoc::MarkdownExtension::FencedDivs,
				pandoc::MarkdownExtension::Smart,
			],
		);
	if let Some(out) = output {
		let _ = pandoc
			.set_output(OutputKind::File(out.to_path_buf()))
			.set_output_format(OutputFormat::Html, vec![]);
	} else {
		let _ = pandoc
			.set_output(OutputKind::Pipe)
			.set_output_format(OutputFormat::Html, vec![]);
	}
	pandoc
}

/// Markdown to HTML converter using Pandoc
#[derive(Default, Debug, Copy, Clone)]
pub struct MarkdownParser;

impl MarkdownParser {
	/// Create new parser instance
	pub fn new() -> Self {
		Self
	}

	/// Extract block components from markdown file
	pub fn extract_components(
		&self,
		input: &Path,
		project_root: Option<&Path>,
	) -> Result<Vec<BlockComponent>, String> {
		let ast = self.to_pandoc_ast(input, project_root)?;
		let mut components = Vec::new();

		for block in ast.blocks {
			let html = self.block_to_html(&block)?;
			components.push(BlockComponent {
				html,
				block: block.clone(),
			});
		}

		Ok(components)
	}

	/// Extract block components with asset extraction
	pub fn extract_components_with_assets(
		&self,
		input: &Path,
		asset_output_dir: &Path,
		asset_slug: &str,
		project_root: Option<&Path>,
	) -> Result<Vec<BlockComponent>, String> {
		let root = project_root
			.map(|p| p.to_path_buf())
			.unwrap_or_else(find_project_root);
		let input_abs = abs_path(input, &root);
		if !input_abs.exists() {
			return Err(format!("Input file not found: {}", input_abs.display()));
		}

		// Create asset output directory
		let asset_dir = asset_output_dir.join(asset_slug);
		std::fs::create_dir_all(&asset_dir)
			.map_err(|e| format!("Failed to create asset dir: {}", e))?;

		// Get AST with rebased paths using markdown input with ExtractMedia
		let mut pandoc = Pandoc::new();
		let _ = pandoc
			.add_options(&[pandoc::PandocOption::ResourcePath(vec![resource_path(&input_abs)]),
				pandoc::PandocOption::ExtractMedia(asset_slug.into())])
			.set_input(pandoc::InputKind::Files(vec![input_abs]))
			.set_input_format(
				pandoc::InputFormat::Markdown,
				vec![
					pandoc::MarkdownExtension::RebaseRelativePaths,
					pandoc::MarkdownExtension::FencedDivs,
					pandoc::MarkdownExtension::Smart,
				],
			)
			.set_output(OutputKind::Pipe)
			.set_output_format(OutputFormat::Json, vec![]);

		// Execute in asset output directory so ExtractMedia works
		let _guard = WithDir::new(asset_output_dir)
			.map_err(|e| format!("Failed to change directory: {}", e))?;

		let ast: PandocAst = match pandoc.execute() {
			Ok(PandocOutput::ToBuffer(json)) => serde_json::from_str(&json)
				.map_err(|e| format!("Failed to parse Pandoc JSON: {}", e))?,
			Ok(_) => return Err("Unexpected Pandoc output kind".into()),
			Err(e) => return Err(format!("Pandoc execution failed: {}", e)),
		};

		// Convert blocks to HTML
		let mut components = Vec::new();
		for block in ast.blocks {
			let html = self.block_to_html(&block)?;
			components.push(BlockComponent {
				html,
				block: block.clone(),
			});
		}

		Ok(components)
	}

	fn block_to_html(&self, block: &Block) -> Result<String, String> {
		// Create minimal Pandoc document with single block
		let doc = PandocAst {
			meta: Default::default(),
			blocks: vec![block.clone()],
		};

		let json =
			serde_json::to_string(&doc).map_err(|e| format!("Failed to serialize block: {}", e))?;

		let mut pandoc = Pandoc::new();
		let _ = pandoc
			.set_input_format(pandoc::InputFormat::Json, vec![])
			.set_input(pandoc::InputKind::Pipe(json))
			.set_output(OutputKind::Pipe)
			.set_output_format(OutputFormat::Html, vec![]);

		match pandoc.execute() {
			Ok(PandocOutput::ToBuffer(html)) => Ok(html),
			Ok(_) => Err("Unexpected Pandoc output kind".into()),
			Err(e) => Err(format!("Pandoc execution failed: {}", e)),
		}
	}

	/// Convert markdown to Pandoc AST
	pub fn to_pandoc_ast(
		&self,
		input: &Path,
		project_root: Option<&Path>,
	) -> Result<PandocAst, String> {
		let root = project_root
			.map(|p| p.to_path_buf())
			.unwrap_or_else(find_project_root);
		let input_abs = abs_path(input, &root);
		if !input_abs.exists() {
			return Err(format!("Input file not found: {}", input_abs.display()));
		}
		debug!("Converting markdown file {:?} to Pandoc AST", input_abs);

		let mut pandoc = Pandoc::new();
		let _ = pandoc
			.add_option(pandoc::PandocOption::ResourcePath(vec![resource_path(
				&input_abs,
			)]))
			.set_input(pandoc::InputKind::Files(vec![input_abs]))
			.set_input_format(
				pandoc::InputFormat::Markdown,
				vec![
					pandoc::MarkdownExtension::RebaseRelativePaths,
					pandoc::MarkdownExtension::FencedDivs,
					pandoc::MarkdownExtension::Smart,
				],
			)
			.set_output(OutputKind::Pipe)
			.set_output_format(OutputFormat::Json, vec![]);

		match pandoc.execute() {
			Ok(PandocOutput::ToBuffer(json)) => serde_json::from_str(&json)
				.map_err(|e| format!("Failed to parse Pandoc JSON: {}", e)),
			Ok(_) => Err("Unexpected Pandoc output kind".into()),
			Err(e) => panic!("Pandoc execution failed: {}", e),
		}
	}

	/// Convert markdown to HTML string
	pub fn to_html_string(
		&self,
		input: &Path,
		project_root: Option<&Path>,
	) -> Result<String, String> {
		let root = project_root
			.map(|p| p.to_path_buf())
			.unwrap_or_else(find_project_root);
		let input_abs = abs_path(input, &root);
		if !input_abs.exists() {
			return Err(format!("Input file not found: {}", input_abs.display()));
		}
		debug!("Converting markdown file {:?} to HTML string", input_abs);
		let pandoc = markdown_pandoc(&input_abs, None, None);
		match pandoc.execute() {
			Ok(PandocOutput::ToBuffer(output)) => Ok(output.to_string()),
			Ok(_) => Err("Unexpected Pandoc output kind".into()),
			Err(e) => panic!("Pandoc execution failed: {}", e),
		}
	}

	/// Convert markdown to HTML file with assets
	pub fn to_html_file(
		&self,
		input: &Path,
		output: &Path,
		asset_slug: &str,
		project_root: Option<&Path>,
	) -> Result<PandocOutput, pandoc::PandocError> {
		let root = project_root
			.map(|p| p.to_path_buf())
			.unwrap_or_else(find_project_root);
		debug!("Using project root: {:?}", root);
		let input_abs = abs_path(input, &root);
		let output_abs = abs_path(output, &root);
		if !input_abs.exists() {
			panic!(
				"Input file not found: {} (resolved from {})",
				input_abs.display(),
				input.display()
			);
		}
		if let Some(parent) = output_abs.parent() {
			std::fs::create_dir_all(parent)
				.unwrap_or_else(|_| panic!("Failed to create output dir: {}", parent.display()));
		}
		let output_asset_dir = output_abs.parent().unwrap().join(asset_slug);
		debug!(
			"Converting markdown file {:?} to HTML file {:?} with assets in {:?}",
			input_abs, output_abs, output_asset_dir
		);
		std::fs::create_dir_all(&output_asset_dir).unwrap();
		let pandoc = markdown_pandoc(&input_abs, Some(&output_abs), Some(asset_slug));
		let _guard =
			WithDir::new(output_abs.parent().unwrap()).expect("Failed to change directory");
		pandoc.execute()
	}
}
