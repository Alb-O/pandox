use std::path::PathBuf;

use clap::Parser;
use log::{error, info};
use parser::MarkdownParser;

#[derive(Parser)]
#[command(name = "parser")]
#[command(about = "Convert markdown to HTML using pandoc_file")]
struct Cli {
	/// Input markdown file
	#[arg(short, long, default_value = "content/test/index.md")]
	input: PathBuf,

	/// Output HTML file
	#[arg(short, long, default_value = "dist/index.html")]
	output: PathBuf,

	/// Asset directory name (relative to output)
	#[arg(short, long, default_value = "assets")]
	assets: String,

	/// Project root directory (defaults to current dir or searches for Cargo.toml)
	#[arg(short, long)]
	root: Option<PathBuf>,
}

fn main() {
	env_logger::builder()
		.format_timestamp(None)
		.format_source_path(true)
		.init();

	let cli = Cli::parse();
	let parser = MarkdownParser::new();

	match parser.to_html_file(&cli.input, &cli.output, &cli.assets, cli.root.as_deref()) {
		Ok(_) => info!("Successfully converted markdown to HTML file"),
		Err(e) => error!("Error during conversion: {}", e),
	}

	match parser.extract_components(&cli.input, cli.root.as_deref()) {
		Ok(components) => {
			info!(
				"Extracted {} block components from markdown",
				components.len()
			);
			for (i, comp) in components.iter().enumerate() {
				info!("Block {}: {:?}", i, comp.block);
				info!("HTML {}: {}", i, comp.html.trim());
			}
		}
		Err(e) => error!("Error extracting components: {}", e),
	}
}
