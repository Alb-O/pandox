use parser::{BlockComponent, MarkdownParser};
use pandoc_types::definition::Block as PandocBlock;
use std::env;
use std::fs;
use std::path::Path;

fn main() {
	println!("cargo:rerun-if-changed=../../content/test/index.md");
	println!("cargo:rerun-if-changed=../../content/test/assets");

	let parser = MarkdownParser::new();
	let input = Path::new("../../content/test/index.md");
	let asset_dir = Path::new("assets");

	let components = parser
		.extract_components_with_assets(input, asset_dir, "test", None)
		.expect("Failed to extract components with assets");

	let out_dir = env::var("OUT_DIR").unwrap();
	let dest_path = Path::new(&out_dir).join("test_content.rs");
	let generated = generate_rsx_code(&components);
	fs::write(&dest_path, generated).expect("Failed to write generated code");

	println!(
		"cargo:warning=Generated {} block components with assets",
		components.len()
	);
}

fn escape_rsx_string(s: &str) -> String {
	// Escape for Rust string literal inside RSX
	s.replace('\\', "\\\\")
		.replace('"', "\\\"")
		.replace('{', "{{")
		.replace('}', "}}")
}

fn generate_rsx_code(components: &[BlockComponent]) -> String {
	let mut code = String::new();
	
	code.push_str("// Auto-generated RSX from content/test/index.md\n");
	code.push_str("use dioxus::prelude::*;\n\n");
	
	code.push_str("#[component]\n");
	code.push_str("pub fn TestContent() -> Element {\n");
	code.push_str("    rsx! {\n");
	
	for (i, comp) in components.iter().enumerate() {
		let class_name = get_block_class_name(&comp);
		// Trim trailing newline from HTML for cleaner formatting
		let html = comp.html.trim_end();
		let html_escaped = escape_rsx_string(html);
		
		code.push_str(&format!("        // Block {}: {}\n", i, get_block_type_name(&comp)));
		code.push_str("        div {\n");
		code.push_str(&format!("            class: \"{}\",\n", class_name));
		code.push_str(&format!("            dangerous_inner_html: \"{}\"\n", html_escaped));
		code.push_str("        }\n");
	}
	
	code.push_str("    }\n");
	code.push_str("}\n");
	
	code
}

fn get_block_type_name(comp: &BlockComponent) -> String {
	match &comp.block {
		PandocBlock::Header(level, _, _) => format!("Header({})", level),
		PandocBlock::Para(_) => "Para".to_string(),
		PandocBlock::CodeBlock(_, _) => "CodeBlock".to_string(),
		PandocBlock::BlockQuote(_) => "BlockQuote".to_string(),
		PandocBlock::BulletList(_) => "BulletList".to_string(),
		PandocBlock::OrderedList(_, _) => "OrderedList".to_string(),
		PandocBlock::Table(_) => "Table".to_string(),
		PandocBlock::Figure(_, _, _) => "Figure".to_string(),
		PandocBlock::Plain(_) => "Plain".to_string(),
		PandocBlock::LineBlock(_) => "LineBlock".to_string(),
		PandocBlock::RawBlock(_, _) => "RawBlock".to_string(),
		PandocBlock::DefinitionList(_) => "DefinitionList".to_string(),
		PandocBlock::HorizontalRule => "HorizontalRule".to_string(),
		PandocBlock::Div(_, _) => "Div".to_string(),
		PandocBlock::Null => "Null".to_string(),
	}
}

fn get_block_class_name(comp: &BlockComponent) -> String {
	match &comp.block {
		PandocBlock::Header(level, _, _) => format!("header header-{}", level),
		PandocBlock::Para(_) => "paragraph".to_string(),
		PandocBlock::CodeBlock(_, _) => "code-block".to_string(),
		PandocBlock::BlockQuote(_) => "blockquote".to_string(),
		PandocBlock::BulletList(_) => "bullet-list".to_string(),
		PandocBlock::OrderedList(_, _) => "ordered-list".to_string(),
		PandocBlock::Table(_) => "table".to_string(),
		PandocBlock::Figure(_, _, _) => "figure".to_string(),
		PandocBlock::Plain(_) => "plain".to_string(),
		PandocBlock::LineBlock(_) => "line-block".to_string(),
		PandocBlock::RawBlock(_, _) => "raw-block".to_string(),
		PandocBlock::DefinitionList(_) => "definition-list".to_string(),
		PandocBlock::HorizontalRule => "horizontal-rule".to_string(),
		PandocBlock::Div(_, _) => "div".to_string(),
		PandocBlock::Null => "null".to_string(),
	}
}
