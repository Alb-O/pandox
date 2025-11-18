//!! Macros for building markdown-based components.

use pandox_markdown::MarkdownParser;
use proc_macro::TokenStream;
use quote::quote;
use log::debug;
use syn::parse_macro_input;
use std::path::PathBuf;

const MODULES_ROOT: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../../modules");

mod args;
mod render;
mod assets;
mod rewrite;
mod dump;
mod utils;

/// Macro to include a markdown file as a Dioxus RSX component.
#[proc_macro]
pub fn mdfile(input: TokenStream) -> TokenStream {
	let args = parse_macro_input!(input as args::MarkdownArgs);

	match expand_markdown(&args) {
		Ok(stream) => stream,
		Err(err) => err,
	}
}

fn expand_markdown(args: &args::MarkdownArgs) -> Result<TokenStream, TokenStream> {
	let crate_root = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap());

	let path_str = args.path.value();

	let markdown_path = utils::resolve_markdown_path(&path_str)?;

	let parser = MarkdownParser::new();
	let _slug = dump::markdown_slug(args, markdown_path.as_path());

	let components = parser
		.extract_components(markdown_path.as_path(), Some(crate_root.as_path()))
		.map_err(|err| {
			let msg = err.to_string();
			TokenStream::from(quote! { compile_error!(#msg); })
		})?;

	let mut asset_ctx = assets::AssetRewriteCtx::new(markdown_path.as_path());

	let mut rendered = Vec::new();
	for block in components {
		rendered.push(render::block_to_tokens(block, &mut asset_ctx));
	}

	dump::dump_full_rsx(markdown_path.as_path(), &rendered);
	debug!("Rendered {} blocks from {:?}", rendered.len(), markdown_path);

	let nodes = rendered.iter().map(|block| block.tokens.clone());

	let expanded = quote! {
		rsx! {
			#(#nodes)*
		}
	};

	Ok(expanded.into())
}
