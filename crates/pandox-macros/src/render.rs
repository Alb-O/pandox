use pandox_pandoc::BlockComponent;
use proc_macro2::TokenStream as TokenStream2;

use crate::assets::AssetRewriteCtx;
use crate::utils::escape_rsx_strings;

pub struct RenderedBlock {
	pub tokens: TokenStream2,
	pub rsx: Option<String>,
}

impl RenderedBlock {
	pub fn empty() -> Self {
		Self {
			tokens: TokenStream2::new(),
			rsx: None,
		}
	}
}

pub fn block_to_tokens(block: BlockComponent, ctx: &mut AssetRewriteCtx) -> RenderedBlock {
	let Some(rsx) = html_to_rsx(&block.html, ctx) else {
		return RenderedBlock::empty();
	};

	let snippet = rsx.trim();
	if snippet.is_empty() {
		return RenderedBlock::empty();
	}

	let escaped = escape_rsx_strings(snippet);

	let tokens = syn::parse_str(&escaped)
		.unwrap_or_else(|err| panic!("failed to parse RSX tokens: {err}\n{escaped}"));

	RenderedBlock {
		tokens,
		rsx: Some(rsx),
	}
}

pub fn html_to_rsx(html: &str, ctx: &mut AssetRewriteCtx) -> Option<String> {
	if html.trim().is_empty() {
		return None;
	}

	let dom =
		html_parser::Dom::parse(html).unwrap_or_else(|err| panic!("failed to parse HTML: {err}"));

	let mut callbody = dioxus_rsx_rosetta::rsx_from_html(&dom);
	crate::rewrite::rewrite_asset_srcs(&mut callbody.body.roots, ctx);

	let formatted = dioxus_autofmt::write_block_out(&callbody)
		.unwrap_or_else(|| panic!("failed to format RSX"));

	if formatted.trim().is_empty() {
		return None;
	}

	Some(formatted)
}
