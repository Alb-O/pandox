use std::path::{Path, PathBuf};

use pandox_markdown::{BlockComponent, MarkdownParser};
use pandox_ui::block_class;
use proc_macro::TokenStream;
use proc_macro2::Span as Span2;
use quote::quote;
use syn::parse::{Parse, ParseStream};
use syn::{Ident, LitStr, Token, parse_macro_input};

struct MarkdownArgs {
	path: LitStr,
	slug: Option<LitStr>,
	asset_dir: Option<LitStr>,
}

impl Parse for MarkdownArgs {
	fn parse(input: ParseStream) -> syn::Result<Self> {
		let path: LitStr = input.parse()?;
		let mut slug = None;
		let mut asset_dir = None;

		while input.peek(Token![,]) {
			input.parse::<Token![,]>()?;
			if input.is_empty() {
				break;
			}

			let key: Ident = input.parse()?;
			input.parse::<Token![=]>()?;
			let value: LitStr = input.parse()?;
			match key.to_string().as_str() {
				"slug" => slug = Some(value),
				"asset_dir" => asset_dir = Some(value),
				other => {
					return Err(syn::Error::new(
						key.span(),
						format!("unknown argument `{other}`"),
					));
				}
			}
		}

		Ok(Self {
			path,
			slug,
			asset_dir,
		})
	}
}

#[proc_macro]
pub fn markdown_component(input: TokenStream) -> TokenStream {
	let args = parse_macro_input!(input as MarkdownArgs);

	match expand_markdown(&args) {
		Ok(stream) => stream,
		Err(err) => err,
	}
}

fn expand_markdown(args: &MarkdownArgs) -> Result<TokenStream, TokenStream> {
	let crate_root = std::env::var("CARGO_MANIFEST_DIR")
		.map(PathBuf::from)
		.map_err(|_| {
			TokenStream::from(
				syn::Error::new(Span2::call_site(), "CARGO_MANIFEST_DIR not set")
					.to_compile_error(),
			)
		})?;

	let path_str = args.path.value();

	let slug = args
		.slug
		.as_ref()
		.map(|s| s.value())
		.unwrap_or_else(|| infer_slug(&path_str));
	let asset_dir = args
		.asset_dir
		.as_ref()
		.map(|s| s.value())
		.unwrap_or_else(|| "assets".to_string());
	let asset_root = resolve_path(&crate_root, &asset_dir);

	let parser = MarkdownParser::new();
	let components = parser
		.extract_components_with_assets(
			Path::new(&path_str),
			asset_root.as_path(),
			&slug,
			Some(crate_root.as_path()),
		)
		.map_err(|err| {
			TokenStream::from(syn::Error::new(Span2::call_site(), err).to_compile_error())
		})?;

	let nodes = components.into_iter().map(block_to_tokens);

	let expanded = quote! {
		rsx! {
			#(#nodes)*
		}
	};

	Ok(expanded.into())
}

fn block_to_tokens(block: BlockComponent) -> proc_macro2::TokenStream {
	let class = block_class(&block);
	let html = LitStr::new(block.html.trim_end(), Span2::call_site());

	match class {
		Some(class) => {
			let class_lit = LitStr::new(&class, Span2::call_site());
			quote! {
				div {
					class: #class_lit,
					dangerous_inner_html: { #html },
				}
			}
		}
		None => quote! {
			div {
				dangerous_inner_html: { #html },
			}
		},
	}
}

fn resolve_path(root: &Path, value: &str) -> PathBuf {
	let candidate = PathBuf::from(value);
	if candidate.is_absolute() {
		candidate
	} else {
		root.join(candidate)
	}
}

fn infer_slug(path: &str) -> String {
	let path = Path::new(path);
	path.parent()
		.and_then(|p| p.file_name())
		.or_else(|| path.file_stem())
		.map(|s| s.to_string_lossy().to_string())
		.unwrap_or_else(|| "content".to_string())
}
