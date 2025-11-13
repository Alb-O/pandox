use std::path::PathBuf;
use log::error;
use clap::Parser;
use parser::MarkdownParser;

#[derive(Parser)]
#[command(name = "parser")]
#[command(about = "Convert markdown to HTML using pandoc")]
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
	let pandoc = parser.to_html_file(&cli.input, &cli.output, &cli.assets, cli.root.as_deref());

	match pandoc.execute() {
		Ok(_) => return,
		Err(e) => error!("Error during conversion: {}", e),
	}
}
